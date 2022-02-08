use crate::boid::Boid;
use nannou::{
    color::Hsv,
    prelude::{vec2, Rect, Vec2},
    rand::random,
    Draw,
};
use nannou_egui::egui::{self, Ui};

#[derive(Debug, Clone, Copy)]
pub struct Ratios {
    pub align: f32,
    pub cohesion: f32,
    pub seperation: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct FlockSettings {
    pub ratios: Ratios,
    pub radius: f32,
    pub top_speed: f32,
    pub max_acc: f32,
}
#[derive(Debug, Clone)]
pub struct Flock {
    name: String,
    boids: Vec<Boid>,
    settings: FlockSettings,
    color: Hsv,
}

impl Flock {
    pub fn new(name: String, color: Hsv) -> Self {
        let boids = (0..100)
            .map(|_x| Boid::new(100.0 * vec2(random::<f32>() - 0.5, random::<f32>() - 0.5)))
            .collect();
        Flock {
            name,
            boids,
            settings: FlockSettings {
                ratios: Ratios {
                    align: 0.5,
                    cohesion: 0.5,
                    seperation: 0.5,
                },
                radius: 40.0,
                top_speed: 10.0,
                max_acc: 0.2,
            },
            color,
        }
    }

    pub fn update(&mut self, rect: Rect, wind_force: Vec2) {
        let boids_copy = self.boids.clone();
        for boid in self.boids.iter_mut() {
            boid.flock(&boids_copy, self.settings);
            boid.apply_force(wind_force);
            boid.update(self.settings.top_speed);
            boid.check_edges(&rect);
        }
    }

    pub fn egui_update(&mut self, ui: &mut Ui) {
        ui.add(egui::Label::new(&self.name));

        nannou_egui::edit_color(ui, &mut self.color);
        ui.add(egui::Slider::new(&mut self.settings.ratios.align, 0.0..=1.0).text("align"))
            .changed();
        ui.add(egui::Slider::new(&mut self.settings.ratios.cohesion, 0.0..=1.0).text("cohesion"))
            .changed();
        ui.add(egui::Slider::new(&mut self.settings.ratios.seperation, 0.0..=1.0).text("sep"))
            .changed();
        ui.add(egui::Slider::new(&mut self.settings.top_speed, 0.0..=100.0).text("top speed"))
            .changed();
        ui.add(egui::Slider::new(&mut self.settings.max_acc, 0.0..=1.0).text("max_acc"))
            .changed();
        ui.add(egui::Slider::new(&mut self.settings.radius, 2.0..=100.0).text("radius"))
            .changed();
    }

    pub fn draw(&self, draw: &Draw) {
        for boid in &self.boids {
            boid.draw(draw, self.color);
        }
    }
}
