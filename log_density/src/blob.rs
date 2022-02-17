use nannou::{
    color::Srgba,
    noise::Perlin,
    prelude::{vec2, Vec2},
};

use crate::{renderer::Renderer, BasicColor, PointParam};


pub type GenPointFn = fn(Vec2, Perlin, &PointParam, &[Srgba]) -> (Vec2, BasicColor);
pub type ExpectationFn = fn(Vec2) -> f32;

impl Blob {
    pub fn new(w: usize, h: usize, noise: Perlin) -> Self {
        let renderer = Renderer::new(w, h);
        Blob { renderer, noise }
    }

    pub fn gen(&mut self, point_param: &PointParam, colors: &[Srgba], gen_point: GenPointFn, expectation: ExpectationFn) {
        let size = self.renderer.w.min(self.renderer.h) as i32;
        (-size..size).for_each(|x| {
            (-size..size).for_each(|y| {
                let xy = vec2(
                    2.0 * (x as f32 / size as f32),
                    2.0 * (y as f32 / size as f32),
                );
                if xy.length() < size as f32 / 2.0 {
                    let (point, color) = gen_point(xy, self.noise, point_param, colors);
                    let expectation = expectation(xy);
                    self.renderer.add(point, color * expectation);
                }
            })
        });
    }
}

pub struct Blob {
    pub renderer: Renderer,
    noise: Perlin,
}
