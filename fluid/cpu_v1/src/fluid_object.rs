use nannou::{
    math::Vec2Angle,
    prelude::{vec2, Rect, Vec2},
    Draw,
};
use ndarray::Array2;

use crate::{advect, advect_vec, diffuse, diffuse_vec, fluid_pos, pos_fluid, project};

pub struct FluidCube {
    dt: f32,
    iter: usize,
    density: Array2<f32>,
    density_prev: Array2<f32>,
    velocity: Array2<Vec2>,
    velocity_prev: Array2<Vec2>,
}

impl FluidCube {
    pub fn new(size: (usize, usize), dt: f32, iter: usize) -> Self {
        Self {
            dt,
            iter,
            density: Array2::zeros(size),
            density_prev: Array2::zeros(size),
            velocity: Array2::from_elem(size, Vec2::ZERO),
            velocity_prev: Array2::from_elem(size, Vec2::ZERO),
        }
    }

    pub fn add_density(&mut self, pos: Vec2, amount: f32, rect: Rect) {
        let v = pos_fluid(pos, rect, self.density.raw_dim());
        self.density[(v.0, v.1)] += amount;
    }

    pub fn add_velocity(&mut self, pos: Vec2, amount: Vec2, rect: Rect) {
        let v = pos_fluid(pos, rect, self.density.raw_dim());
        self.velocity[(v.0, v.1)] += amount;
    }

    fn dens_step(&mut self, visc: f32) {
        diffuse(
            &mut self.density,
            &self.density_prev,
            self.iter,
            self.dt,
            visc,
        );
        let new_density = advect(&self.density, &self.velocity, self.dt);

        std::mem::swap(&mut self.density_prev, &mut self.density);
        self.density = new_density;
    }

    fn vel_step(&mut self, diff: f32) {
        diffuse_vec(
            &mut self.velocity,
            &self.velocity_prev,
            self.iter,
            self.dt,
            diff,
        );
        project(&mut self.velocity, self.iter);
        let mut new_velocity = advect_vec(&self.velocity_prev, &self.velocity_prev, self.dt);
        project(&mut new_velocity, self.iter);

        std::mem::swap(&mut self.velocity_prev, &mut self.velocity);
        self.velocity = new_velocity;
    }

    pub fn step(&mut self, diff: f32, visc: f32) {
        self.vel_step(diff);
        self.dens_step(visc);
    }

    pub fn draw_dens(&self, draw: &Draw, wrect: Rect) {
        let dim = self.density.raw_dim();
        let step = wrect.wh() / vec2(dim[0] as f32, dim[1] as f32);

        for x in 0..dim[0] {
            for y in 0..dim[1] {
                let inc = fluid_pos((x, y), wrect, dim);

                draw.rect()
                    .xy(inc)
                    .wh(step)
                    .hsl(0.5, 0.7, self.density[(x, y)]);
            }
        }
    }

    pub fn draw_vel(&self, draw: &Draw, wrect: Rect) {
        let dim = self.velocity.raw_dim();

        for x in 0..dim[0] {
            for y in 0..dim[1] {
                let inc = fluid_pos((x, y), wrect, dim);
                let vec = self.velocity[(x, y)];
                draw.line()
                    .points(inc, inc + 5.0 * vec2(vec.angle().cos(), vec.angle().sin()))
                    .rgb(1.0, 1.0, 1.0);
            }
        }
    }
}
