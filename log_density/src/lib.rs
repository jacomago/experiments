use std::ops::{AddAssign, Mul};

use nannou::{color::Gradient, prelude::*};
use nannou_egui::egui;

use nannou_egui::egui::Ui;

pub mod blob;
pub mod renderer;

#[derive(Debug)]
pub struct PointParam {
    pub zero_point: Vec2,
    pub noise_pos: Vec2,
    pub noise_scale: f32,
    pub scale: f32,
}

impl PointParam {
    pub fn ui_update(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;
        egui::CollapsingHeader::new("Point Param").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("noise xy");
                changed = changed
                    || ui
                        .add(egui::DragValue::new(&mut self.noise_pos.x).speed(0.1))
                        .changed();
                changed = changed
                    || ui
                        .add(egui::DragValue::new(&mut self.noise_pos.y).speed(0.1))
                        .changed();
            });
            ui.horizontal(|ui| {
                ui.label("zero_point xy");
                changed = changed
                    || ui
                        .add(egui::DragValue::new(&mut self.zero_point.x).speed(0.1))
                        .changed();
                changed = changed
                    || ui
                        .add(egui::DragValue::new(&mut self.zero_point.y).speed(0.1))
                        .changed();
            });
            changed = changed
                || ui
                    .add(egui::Slider::new(&mut self.scale, 100.0..=1000.0_f32).text("scale"))
                    .changed();
            changed = changed
                || ui
                    .add(egui::Slider::new(&mut self.noise_scale, 10.0..=200.0).text("noise scale"))
                    .changed();
        });
        changed
    }
}
#[derive(Copy, Clone, Debug)]
pub struct BasicColor {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

pub const ZERO: BasicColor = BasicColor {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

impl AddAssign for BasicColor {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            alpha: self.alpha + other.alpha,
        };
    }
}

impl Mul<f32> for BasicColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
            alpha: self.alpha * rhs,
        }
    }
}
pub fn basic_color(c: Srgba) -> BasicColor {
    BasicColor {
        red: c.red,
        green: c.green,
        blue: c.blue,
        alpha: c.alpha,
    }
}

pub fn lerp_colors_duo(back: Srgba, fore: Srgba, alpha: f32) -> Srgba {
    let grad = Gradient::new(vec![back.into_linear(), fore.into_linear()]);
    Srgba::from_linear(grad.get(alpha))
}

pub fn lerp_colors(colors: &[Srgba], alpha: f32) -> Srgba {
    let grad = Gradient::new(colors.iter().map(|c| c.into_linear()));
    Srgba::from_linear(grad.get(alpha))
}

pub fn color_u8(c: Srgba) -> [u8; 4] {
    [
        map_range(c.red, 0.0, 1.0, 0, std::u8::MAX),
        map_range(c.green, 0.0, 1.0, 0, std::u8::MAX),
        map_range(c.blue, 0.0, 1.0, 0, std::u8::MAX),
        map_range(c.alpha, 0.0, 1.0, 0, std::u8::MAX),
    ]
}
