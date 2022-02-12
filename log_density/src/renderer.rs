use nannou::{
    image::{self, RgbaImage},
    prelude::*,
};

use crate::{lerp_colors_duo, color_u8};

#[derive(Copy, Clone)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: u32,
}

const ZERO: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0,
};

pub struct Renderer {
    hits: Vec<Vec<Color>>,
    w: usize,
    h: usize,
    img: RgbaImage,
}



impl Renderer {
    pub fn new(w: usize, h: usize) -> Self {
        let hits = vec![vec![ZERO; w]; h];
        let img = RgbaImage::new(w as u32, h as u32);
        Renderer { hits, w, h, img }
    }

    pub fn add(&mut self, xy: Vec2, color: Srgba) {
        let xint = xy.x.floor() as i32;
        let yint = xy.y.floor() as i32;
        if xint >= 0 && xint < self.w as i32 && yint >= 0 && yint < self.h as i32 {
            self.hits[yint as usize][xint as usize].alpha += 1;
            self.hits[yint as usize][xint as usize].red += color.red;
            self.hits[yint as usize][xint as usize].green += color.green;
            self.hits[yint as usize][xint as usize].blue += color.blue;
        }
    }

    pub fn render(&mut self, back: Srgba, gamma: f32) {
        let mx = self
            .hits
            .iter()
            .map(|v| v.iter().map(|x| x.alpha).max())
            .max()
            .unwrap()
            .unwrap() as f32;
        let mx = (mx + 1.0).ln();
        for (x, row) in self.hits.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                let pixel = if col.alpha > 0 {
                    let hits = col.alpha as f32;
                    let alpha = ((hits + 1.0).ln() / mx).pow(gamma);
                    let fore = srgba(col.red / hits, col.green / hits, col.blue / hits, 1.0);
                    let new_c = lerp_colors_duo(back, fore, alpha);
                    image::Rgba(color_u8(new_c))
                } else {
                    image::Rgba(color_u8(back))
                };

                self.img.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
    pub fn img(&self) -> &RgbaImage {
        &self.img
    }
}
