use nannou::{
    image::{self, RgbaImage},
    prelude::*,
};

use crate::{color_u8, lerp_colors_duo};

#[derive(Copy, Clone)]
struct BasicColor {
    red: f32,
    green: f32,
    blue: f32,
    alpha: u32,
}

const ZERO: BasicColor = BasicColor {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0,
};

pub struct Renderer {
    hits: Vec<Vec<BasicColor>>,
    w: usize,
    h: usize,
    img: RgbaImage,
}

pub struct ColorSettings {
    gamma: f32,
    cgamma: f32,
    color_mix: Option<f32>,
    brightness: Option<f32>,
    contrast: Option<f32>,
    saturation: Option<f32>,
}

impl ColorSettings {
    pub fn new(
        gamma: f32,
        cgamma: f32,
        color_mix: Option<f32>,
        brightness: Option<f32>,
        contrast: Option<f32>,
        saturation: Option<f32>,
    ) -> Self {
        ColorSettings {
            gamma,
            cgamma,
            color_mix,
            brightness,
            contrast,
            saturation,
        }
    }
}

fn pixel_calc(
    x: u32,
    y: u32,
    col: BasicColor,
    mx: f32,
    color_settings: &ColorSettings,
    back: Srgba,
) -> image::Rgba<u8> {
     if col.alpha > 0 {
        let hits = col.alpha as f32;
        let alpha = ((hits + 1.0).ln() / mx).pow(color_settings.gamma);
        let fore = srgba(
            col.red / hits,
            col.green / hits,
            col.blue / hits,
            1.0,
        );
        let new_c = lerp_colors_duo(back, fore, alpha);
        image::Rgba(color_u8(new_c))
    } else {
        image::Rgba(color_u8(back))
    }

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

    pub fn render(&mut self, back: Srgba, color_settings: ColorSettings) {
        let mx = self
            .hits
            .iter()
            .map(|v| v.iter().map(|x| x.alpha).max())
            .max()
            .unwrap()
            .unwrap() as f32;
        let mx = (mx + 1.0).ln();

        self.img = image::ImageBuffer::from_fn(self.w.try_into().unwrap(), self.h.try_into().unwrap(), |x, y| {
            pixel_calc(x, y, self.hits[x as usize][y as usize],mx, &color_settings, back)
        });
    }
    pub fn img(&self) -> &RgbaImage {
        &self.img
    }
}
