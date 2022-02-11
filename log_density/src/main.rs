use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou::rand::prelude::ThreadRng;
use nannou::rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

fn main() {
    nannou::app(model).update(update).run();
}

struct Blob {
    points: Vec<Vec2>,
    rng: ThreadRng,
    noise: Perlin,
}

fn gen_point(
    noise_scale: f32,
    rng: &mut ThreadRng,
    noise: Perlin,
    perlin_factor: f64,
    scale: f32,
) -> Vec2 {
    let xy = vec2(rng.sample(StandardNormal), rng.sample(StandardNormal));
    let r = noise_scale * xy.length();
    let nxy = r * vec2(
        (noise.get([xy.x as f64, xy.y as f64]) - 0.5) as f32,
        (noise.get([(xy.y - 1.1) as f64, (xy.x + 1.1) as f64, perlin_factor]) - 0.5) as f32,
    );
    scale * xy + nxy
}

impl Blob {
    fn new(_int: usize, rng: ThreadRng, noise: Perlin) -> Self {
        let points = Vec::new();
        Blob { points, rng, noise }
    }
    fn new_point(&mut self, noise_scale: f32, perlin_factor: f64, scale: f32) {
        self.points.push(gen_point(
            noise_scale,
            &mut self.rng,
            self.noise,
            perlin_factor,
            scale,
        ));
    }
    fn new_points(&mut self, noise_scale: f32, perlin_factor: f64, scale: f32, amount: usize) {
        (0..amount).for_each(|f| self.new_point(noise_scale, perlin_factor, scale));
    }
    fn last_point(&self) -> Vec2 {
        *self.points.last().unwrap_or(&vec2(0.0, 0.0))
    }
    fn last_points(&self, amount: usize) -> Vec<Vec2> {
        (*self
            .points
            .chunks_exact(amount)
            .last()
            .unwrap_or(&[Vec2::ZERO]))
        .to_vec()
    }
}
struct Model {
    blob: Blob,
    rate: usize,
    scale: f32,
    perlin_factor: f64,
    noise_scale: f32
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.scale, &mut model.noise_scale, key);
}
const SIZE: usize = 500;
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let rng = thread_rng();
    let noise = Perlin::new();
    let blob = Blob::new(0,  rng, noise);
    let rate = 100;
    Model { blob, rate , scale: 100.0, perlin_factor: 0.4, noise_scale: 50.0}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.blob.new_points(model.noise_scale, model.perlin_factor, model.scale, model.rate);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }
    for point in model.blob.last_points(model.rate) {
        draw.ellipse()
            .xy(point)
            .radius(1.0)
            .rgba(1.0, 1.0, 1.0, 0.9);
    }
    draw.to_frame(app, &frame).unwrap();
}
