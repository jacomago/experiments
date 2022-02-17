use nannou::{
    image::{self, RgbaImage},
    prelude::*,
};
use nannou_egui::egui::{self, Ui};

use crate::{color_u8, lerp_colors_duo, BasicColor, ZERO};

pub struct Renderer {
    hits: Vec<Vec<BasicColor>>,
    pub w: usize,
    pub h: usize,
    img: RgbaImage,
}

#[derive(Debug)]
pub struct BrightnessContrast {
    brightness: f32,
    contrast: f32,
}

impl From<(f32, f32)> for BrightnessContrast {
    fn from(value: (f32, f32)) -> Self {
        BrightnessContrast {
            brightness: value.0,
            contrast: value.1,
        }
    }
}
#[derive(Debug)]
pub struct ColorMix {
    fore: f32,
    back: f32,
}
impl From<(f32, f32)> for ColorMix {
    fn from(value: (f32, f32)) -> Self {
        ColorMix {
            fore: value.0,
            back: value.1,
        }
    }
}

#[derive(Debug)]
pub struct ColorSettings {
    gamma: f32,
    color_mix: Option<ColorMix>,
    brightness_contrast: Option<BrightnessContrast>,
    saturation: Option<f32>,
}

impl ColorSettings {
    pub fn new(
        gamma: f32,
        color_mix: Option<(f32, f32)>,
        brightness_contrast: Option<(f32, f32)>,
        saturation: Option<f32>,
    ) -> Self {
        let brightness_contrast = brightness_contrast.map(BrightnessContrast::from);
        let color_mix = color_mix.map(ColorMix::from);
        ColorSettings {
            gamma,
            color_mix,
            brightness_contrast,
            saturation,
        }
    }

    pub fn ui_update(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        egui::CollapsingHeader::new("Color Settings").show(ui, |ui| {
            changed = changed
                || ui
                    .add(egui::Slider::new(&mut self.gamma, 0.0..=4.0).text("gamma"))
                    .changed();
            match &mut self.brightness_contrast {
                Some(value) => {
                    changed = changed
                        || ui
                            .add(
                                egui::Slider::new(&mut value.brightness, 0.0..=4.0)
                                    .text("brightness"),
                            )
                            .changed();
                    changed = changed
                        || ui
                            .add(egui::Slider::new(&mut value.contrast, 0.0..=4.0).text("contrast"))
                            .changed();
                }
                None => {
                    if ui
                        .add(egui::Checkbox::new(&mut false, "brightness contrast"))
                        .changed()
                    {
                        self.brightness_contrast = Some(BrightnessContrast {
                            brightness: 1.0,
                            contrast: 1.0,
                        });
                        changed = true;
                    }
                }
            };
        });
        changed
    }
}

fn mix_colors(fore: Srgba, color_mix: &ColorMix) -> Srgba {
    let gamma_correct = srgba(
        fore.red.pow(color_mix.back),
        fore.green.pow(color_mix.back),
        fore.blue.pow(color_mix.back),
        1.0,
    );
    lerp_colors_duo(fore, gamma_correct, color_mix.fore)
}

fn lut_f32(c_value: f32, amount: &BrightnessContrast) -> f32 {
    ((c_value * amount.brightness - 0.5) * amount.contrast + 0.5).clamp(0.0, 1.0)
}

fn lut(c: Srgba, amount: &BrightnessContrast) -> Srgba {
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
    let mut calc_color = if col.alpha > 0.0 {
        // convert hits to float
        let hits = col.alpha as f32;

        // changed to log scale
        let alpha = ((hits + 1.0).ln() / mx).pow(color_settings.gamma);

        // avg color
        let fore = srgba(col.red / hits, col.green / hits, col.blue / hits, 1.0);

        // linear interpolate colors
        if let Some(color_mix) = &color_settings.color_mix {
            lerp_colors_duo(back, mix_colors(fore, color_mix), alpha)
        } else {
            lerp_colors_duo(back, fore, alpha)
        }
    } else {
        back
    };

    if let Some(bc) = &color_settings.brightness_contrast {
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

    pub fn add(&mut self, xy: Vec2, color: BasicColor) {
        let xint = xy.x.floor() as i32;
        let yint = xy.y.floor() as i32;
        if xint >= 0 && xint < self.w as i32 && yint >= 0 && yint < self.h as i32 {
            self.hits[yint as usize][xint as usize] += color;
        }
    }

    pub fn render(&mut self, back: Srgba, color_settings: &ColorSettings) {
        // max hits
        let mut max = 0.0;
        for row in &self.hits {
            for col in row {
                if col.alpha > max {
                    max = col.alpha;
                }
            }
        }

        // log scale
        let mx = (max + 1.0).ln();

        // create image from hits
        self.img = image::ImageBuffer::from_fn(
            self.w.try_into().unwrap(),
            self.h.try_into().unwrap(),
            |x, y| pixel_calc(self.hits[x as usize][y as usize], mx, color_settings, back),
        );
    }

    pub fn img(&self) -> &RgbaImage {
        &self.img
    }
}
