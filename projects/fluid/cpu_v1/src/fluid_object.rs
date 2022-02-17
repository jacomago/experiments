use nannou::{
    color::{hsv, Hsv},
    math::Vec2Angle,
    prelude::{vec2, Rect, Vec2},
    Draw,
};
use ndarray::Array2;

use crate::{advect, advect_vec, diffuse, diffuse_vec, fluid_pos, pos_fluid, project};

#[derive(Copy, Clone, Debug)]
pub struct DensColor {
    pub hue: f32,
    pub sat: f32,
}

impl DensColor {
    pub fn new(hue: f32, sat: f32) -> Self {
        DensColor { hue, sat }
    }

    fn color_map(&self, ratio: f32) -> Hsv {
        hsv(self.hue, self.sat, ratio.clamp(0.0, 1.0))
    }
}

pub struct FluidCube {
    density: Array2<f32>,
    density_prev: Array2<f32>,
    velocity: Array2<Vec2>,
    velocity_prev: Array2<Vec2>,
}

impl FluidCube {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
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

    fn dens_step(&mut self, visc: f32, dt: f32, iter: usize) {
        diffuse(&mut self.density, &self.density_prev, iter, dt, visc);
        let new_density = advect(&self.density, &self.velocity, dt);

        std::mem::swap(&mut self.density_prev, &mut self.density);
        self.density = new_density;
    }

    fn vel_step(&mut self, diff: f32, dt: f32, iter: usize) {
        diffuse_vec(&mut self.velocity, &self.velocity_prev, iter, dt, diff);
        project(&mut self.velocity, iter);
        let mut new_velocity = advect_vec(&self.velocity_prev, &self.velocity_prev, dt);
        project(&mut new_velocity, iter);

        std::mem::swap(&mut self.velocity_prev, &mut self.velocity);
        self.velocity = new_velocity;
    }

    pub fn step(&mut self, vel_diff: f32, dens_visc: f32, dt: f32, iter: usize) {
        self.vel_step(vel_diff, dt, iter);
        self.dens_step(dens_visc, dt, iter);
    }

    pub fn draw_dens(&self, draw: &Draw, wrect: Rect, density_color: DensColor) {
        let dim = self.density.raw_dim();
        let step = wrect.wh() / vec2(dim[0] as f32, dim[1] as f32);

        for x in 0..dim[0] {
            for y in 0..dim[1] {
                let inc = fluid_pos((x, y), wrect, dim);

                draw.rect()
                    .xy(inc)
                    .wh(step)
                    .color(density_color.color_map(self.density[(x, y)]));
            }
        }
    }

    pub fn draw_vel(&self, draw: &Draw, wrect: Rect, line_length: f32, color: Hsv) {
        let dim = self.velocity.raw_dim();

        for x in 0..dim[0] {
            for y in 0..dim[1] {
                let inc = fluid_pos((x, y), wrect, dim);
                let vec = self.velocity[(x, y)];
                draw.line()
                    .points(
                        inc,
                        inc + line_length * vec2(vec.angle().cos(), vec.angle().sin()),
                    )
                    .color(color);
            }
        }
    }
}
