use nannou::{
    math::map_range,
    prelude::{vec2, Rect, Vec2},
};
use ndarray::{Array2, Dim};

fn neighbours(pair: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (pair.0 - 1, pair.1),
        (pair.0 + 1, pair.1),
        (pair.0, pair.1 - 1),
        (pair.0, pair.1 + 1),
    ]
}

/// On the boundaries of the array we set some condition,
/// such as disapation
fn set_boundries<A: std::ops::Add<Output = A> + std::ops::Mul<f32, Output = A> + Copy>(
    array: &mut Array2<A>,
) {
    let raw_dim = array.raw_dim();
    array[(0, 0)] = (array[(1, 0)] + array[(0, 1)]) * 0.5;
    array[(0, raw_dim[1] - 1)] = (array[(1, raw_dim[1] - 1)] + array[(0, raw_dim[1] - 2)]) * 0.5;
    array[(raw_dim[0] - 1, 0)] = (array[(raw_dim[0] - 1, 1)] + array[(raw_dim[0] - 2, 0)]) * 0.5;
    array[(raw_dim[0] - 1, raw_dim[1] - 1)] =
        (array[(raw_dim[0] - 1, raw_dim[1] - 2)] + array[(raw_dim[0] - 2, raw_dim[1] - 1)]) * 0.5;
}

/// Diffuses the array by making it the average sum of it's neighbours.
/// Using Gauss-Seidel relaxation to solve a system of linear equations of
/// form
/// x0[IX(i,j)] = x[IX(i,j)] - a*(x[IX(i-1,j)]+x[IX(i+1,j)]+x[IX(i,j-1)]+x[IX(i,j+1)]-4*x[IX(i,j)])
///
fn diffuse(array: &mut Array2<f32>, array_prev: &Array2<f32>, iter: usize, dt: f32, visc: f32) {
    let shape = array_prev.shape();
    let multiplier = dt * visc * shape[0] as f32 * shape[1] as f32;
    for _ in 0..iter {
        for x in 1..(shape[0] - 1) {
            for y in 1..(shape[1] - 1) {
                let ns = neighbours((x, y));

                let sum: f32 = ns.iter().map(|n| array[(n.0, n.1)]).sum();

                array[(x, y)] = (array_prev[(x, y)] + multiplier * sum)
                    * (1.0 / (1.0 + ns.len() as f32 * multiplier));
            }
        }
        set_boundries(array);
    }
}

/// Trace backwards with linear interpolation to sources of current density
/// then set value as a suitable average of the nearby densities of the past
fn advect(density_prev: &Array2<f32>, velocity: &Array2<Vec2>, dt: f32) -> Array2<f32> {
    let shape = density_prev.shape();
    let mut output: Array2<f32> = Array2::zeros((shape[0], shape[1]));
    let v_shape = vec2(shape[0] as f32, shape[1] as f32);
    let dt_shape = dt * v_shape;
    for x in 1..(shape[0] - 1) {
        for y in 1..(shape[1] - 1) {
            let el = ((x, y), density_prev[(x, y)]);
            let pos = vec2(el.0 .0 as f32, el.0 .1 as f32) - dt_shape * velocity[el.0];
            let pos_clamp = pos.clamp(vec2(0.5, 0.5), v_shape + vec2(0.5, 0.5));
            let pos_floor = vec2(pos_clamp.x.floor(), pos_clamp.y.floor());
            let pos_frac = pos_clamp - pos_floor;
            let neg_frac = vec2(1.0, 1.0) - pos_frac;

            let index_pos = (pos_floor.x as usize, pos_floor.y as usize);

            output[el.0] = neg_frac.x
                * (neg_frac.y * density_prev[index_pos]
                    + pos_frac.y * density_prev[(index_pos.0, index_pos.1 + 1)])
                + pos_frac.x
                    * (neg_frac.y * density_prev[(index_pos.0 + 1, index_pos.1)]
                        + pos_frac.y * density_prev[(index_pos.0 + 1, index_pos.1 + 1)]);
        }
    }
    set_boundries(&mut output);
    output
}

/// On the boundaries of the array we set some condition,
/// such as disapation
fn set_boundries_vec(array: &mut Array2<Vec2>) {
    // edge columns
    let dim = array.raw_dim();
    for i in 1..(dim[0] - 1) {
        array[(i, 0)] = vec2(array[(i, 1)].x, -array[(i, 1)].y);
        array[(i, dim[1] - 1)] = vec2(array[(i, dim[1] - 2)].x, -array[(i, dim[1] - 2)].y);
    }
    for i in 1..(dim[1] - 1) {
        array[(0, i)] = vec2(-array[(1, i)].x, array[(1, i)].y);
        array[(dim[1] - 1, i)] = vec2(-array[(dim[0] - 2, i)].x, array[(dim[0] - 2, i)].y);
    }
    set_boundries(array);
}

/// Diffuses the array by making it the average sum of it's neighbours.
/// Using Gauss-Seidel relaxation to solve a system of linear equations of
/// form
/// x0[IX(i,j)] = x[IX(i,j)] - a*(x[IX(i-1,j)]+x[IX(i+1,j)]+x[IX(i,j-1)]+x[IX(i,j+1)]-4*x[IX(i,j)])
///
fn diffuse_vec(
    array: &mut Array2<Vec2>,
    array_prev: &Array2<Vec2>,
    iter: usize,
    dt: f32,
    visc: f32,
) {
    let shape = array_prev.shape();
    let multiplier = dt * visc * shape[0] as f32 * shape[1] as f32;
    for _ in 0..iter {
        for x in 1..(shape[0] - 1) {
            for y in 1..(shape[1] - 1) {
                let ns = neighbours((x, y));

                let mut sum = Vec2::ZERO;
                ns.iter().for_each(|n| sum += array[(n.0, n.1)]);

                array[(x, y)] = (array_prev[(x, y)] + multiplier * sum)
                    * (1.0 / (1.0 + ns.len() as f32 * multiplier));
            }
        }
        set_boundries_vec(array);
    }
}

/// Trace backwards with linear interpolation to sources of current density
/// then set value as a suitable average of the nearby densities of the past
fn advect_vec(velocity_prev: &Array2<Vec2>, velocity: &Array2<Vec2>, dt: f32) -> Array2<Vec2> {
    let shape = velocity_prev.raw_dim();
    let mut output: Array2<Vec2> = Array2::from_elem(shape, Vec2::ZERO);
    let v_shape = vec2(shape[0] as f32, shape[1] as f32);
    let dt_shape = dt * v_shape;
    for x in 1..(shape[0] - 1) {
        for y in 1..(shape[1] - 1) {
            let el = ((x, y), velocity_prev[(x, y)]);
            let pos = vec2(el.0 .0 as f32, el.0 .1 as f32) - dt_shape * velocity[el.0];
            let pos_clamp = pos.clamp(vec2(0.5, 0.5), v_shape + vec2(0.5, 0.5));
            let pos_floor = vec2(pos_clamp.x.floor(), pos_clamp.y.floor());
            let pos_frac = pos_clamp - pos_floor;
            let neg_frac = vec2(1.0, 1.0) - pos_frac;

            let index_pos = (pos_floor.x as usize, pos_floor.y as usize);

            output[el.0] = neg_frac.x
                * (neg_frac.y * velocity_prev[index_pos]
                    + pos_frac.y * velocity_prev[(index_pos.0, index_pos.1 + 1)])
                + pos_frac.x
                    * (neg_frac.y * velocity_prev[(index_pos.0 + 1, index_pos.1)]
                        + pos_frac.y * velocity_prev[(index_pos.0 + 1, index_pos.1 + 1)]);
        }
    }
    set_boundries_vec(&mut output);
    output
}

fn project(velocity: &mut Array2<Vec2>, iter: usize) {
    let shape = velocity.raw_dim();
    let size_recip = vec2((shape[0] as f32).recip(), (shape[1] as f32).recip());

    let mut div: Array2<f32> = Array2::zeros(shape);

    for x in 1..(shape[0] - 1) {
        for y in 1..(shape[1] - 1) {
            div[(x, y)] = -0.5
                * (size_recip.x * (velocity[(x + 1, y)].x - velocity[(x - 1, y)].x)
                    + size_recip.y * (velocity[(x, y + 1)].y - velocity[(x, y - 1)].y));
        }
    }
    set_boundries(&mut div);

    let mut p: Array2<f32> = Array2::zeros(shape);
    for _ in 0..iter {
        for x in 1..(shape[0] - 1) {
            for y in 1..(shape[1] - 1) {
                p[(x, y)] = 0.25
                    * (div[(x, y)] + p[(x + 1, y)] - p[(x - 1, y)] + p[(x, y + 1)] - p[(x, y - 1)]);
            }
        }
        set_boundries(&mut p);
    }

    for x in 1..(shape[0] - 1) {
        for y in 1..(shape[1] - 1) {
            velocity[(x, y)] = 0.5
                * vec2(
                    size_recip.x * (p[(x + 1, y)] - p[(x - 1, y)]),
                    size_recip.y * (p[(x, y + 1)] - p[(x, y - 1)]),
                );
        }
    }
    set_boundries_vec(velocity);
}

fn fluid_pos(pos: (usize, usize), rect: Rect, dim: Dim<[usize; 2]>) -> Vec2 {
    let inc_x = map_range(pos.0, 0, dim[0], rect.left(), rect.right());
    let inc_y = map_range(pos.1, 0, dim[1], rect.bottom(), rect.top());
    vec2(inc_x, inc_y)
}

fn pos_fluid(pos: Vec2, rect: Rect, dim: Dim<[usize; 2]>) -> (usize, usize) {
    let inc_x = map_range(pos.x, rect.left(), rect.right(), 0, dim[0]);
    let inc_y = map_range(pos.y, rect.bottom(), rect.top(), 0, dim[1]);
    (inc_x, inc_y)
}

pub mod fluid_object;
