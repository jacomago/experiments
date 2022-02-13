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
    color_mix: Option<(f32, f32)>,
    brightness_contrast: Option<(f32, f32)>,
    saturation: Option<f32>,
}

impl ColorSettings {
    pub fn new(
        gamma: f32,
        color_mix: Option<(f32, f32)>,
        brightness_contrast: Option<(f32, f32)>,
        saturation: Option<f32>,
    ) -> Self {
        ColorSettings {
            gamma,
            color_mix,
            brightness_contrast,
            saturation,
        }
    }
}

fn mix_colors(fore: Srgba, color_mix: (f32, f32)) -> Srgba {
    let gamma_correct = srgba(
        fore.red.pow(color_mix.1),
        fore.green.pow(color_mix.1),
        fore.blue.pow(color_mix.1),
        1.0,
    );
    lerp_colors_duo(fore, gamma_correct, color_mix.0)
}

fn lut_f32(c_value: f32, amount: (f32, f32)) -> f32 {
    ((c_value * amount.0 - 0.5) * amount.1 + 0.5).clamp(0.0, 1.0)
}

fn lut(c: Srgba, amount: (f32, f32)) -> Srgba {
    srgba(
        lut_f32(c.red, amount),
        lut_f32(c.green, amount),
        lut_f32(c.blue, amount),
        c.alpha,
    )
}

fn saturate(c: Srgba, amount: f32) -> Srgba {
    let hsb: Hsla = c.into();
    Hsla::new(
        hsb.hue,
        (hsb.saturation * amount).clamp(0.0, 1.0),
        hsb.lightness,
        hsb.alpha,
    )
    .into()
}

fn pixel_calc(
    col: BasicColor,
    mx: f32,
    color_settings: &ColorSettings,
    back: Srgba,
) -> image::Rgba<u8> {
    let mut calc_color = if col.alpha > 0 {
        // convert hits to float
        let hits = col.alpha as f32;

        // changed to log scale
        let alpha = ((hits + 1.0).ln() / mx).pow(color_settings.gamma);

        // avg color
        let fore = srgba(col.red / hits, col.green / hits, col.blue / hits, 1.0);

        // linear interpolate colors
        if let Some(color_mix) = color_settings.color_mix {
            lerp_colors_duo(back, mix_colors(fore, color_mix), alpha)
        } else {
            lerp_colors_duo(back, fore, alpha)
        }
    } else {
        back
    };

    if let Some(bc) = color_settings.brightness_contrast {
        calc_color = lut(calc_color, bc)
    };

    if let Some(sat) = color_settings.saturation {
        calc_color = saturate(calc_color, sat)
    };

    image::Rgba(color_u8(calc_color))
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
        // max hits
        let mx = self
            .hits
            .iter()
            .map(|v| v.iter().map(|x| x.alpha).max())
            .max()
            .unwrap()
            .unwrap() as f32;

        // log scale
        let mx = (mx + 1.0).ln();

        // create image from hits
        self.img = image::ImageBuffer::from_fn(
            self.w.try_into().unwrap(),
            self.h.try_into().unwrap(),
            |x, y| pixel_calc(self.hits[x as usize][y as usize], mx, &color_settings, back),
        );
    }

    pub fn img(&self) -> &RgbaImage {
        &self.img
    }
}
