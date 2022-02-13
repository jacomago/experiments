use log_density::lerp_colors;
use log_density::renderer::{ColorSettings, Renderer};
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou::rand::prelude::ThreadRng;
use nannou::rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

fn main() {
    nannou::app(model).run();
}

struct Blob {
    renderer: Renderer,
    rng: ThreadRng,
    noise: Perlin,
}

fn gen_point(
    noise_scale: f32,
    rng: &mut ThreadRng,
    noise: Perlin,
    perlin_factor: f64,
    scale: f32,
    colors: &[Srgba],
) -> (Vec2, Srgba) {
    let xy = vec2(rng.sample(StandardNormal), rng.sample(StandardNormal));
    let r = noise_scale * xy.length();
    let t = vec2(
        (noise.get([xy.x as f64, xy.y as f64]) - 0.5) as f32,
        (noise.get([(xy.y - 1.1) as f64, (xy.x + 1.1) as f64, perlin_factor]) - 0.5) as f32,
    );
    let nxy = r * t;
    (
        vec2(500.0, 500.0) + scale * xy + nxy,
        lerp_colors(colors, t.length()),
    )
}

struct Model {
    blob: Blob,
    texture: wgpu::Texture,
    rate: usize,
    scale: f32,
    perlin_factor: f64,
    noise_scale: f32,
    colors: Vec<Srgba>,
}

impl Blob {
    fn new(w: usize, h: usize, rng: ThreadRng, noise: Perlin) -> Self {
        let renderer = Renderer::new(w, h);
        Blob {
            renderer,
            rng,
            noise,
        }
    }

    fn gen(
        &mut self,
        rate: usize,
        noise_scale: f32,
        perlin_factor: f64,
        scale: f32,
        colors: &[Srgba],
    ) {
        (0..rate).for_each(|_i| {
            let (point, color) = gen_point(
                noise_scale,
                &mut self.rng,
                self.noise,
                perlin_factor,
                scale,
                colors,
            );
            self.renderer.add(point, color)
        });
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.scale, &mut model.noise_scale, key);

    match key {
        Key::G => {
            model.blob.gen(
                model.rate,
                model.noise_scale,
                model.perlin_factor,
                model.scale,
                &model.colors,
            );
        }
        _other_key => {}
    }
}

const SIZE: usize = 1000;

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

    let rate = 1000000;
    let scale = 200.0;
    let perlin_factor = 0.4;
    let noise_scale = 50.0;

    let colors = vec![
        srgba(0.76, 0.66, 0.9, 1.0),
        srgba(0.5, 0.0, 0.7, 1.0),
        srgba(0.0, 0.0, 0.5, 1.0),
    ];

    let wrect = app.window_rect();

    let window = app.main_window();
    let texture = wgpu::TextureBuilder::new()
        .size([wrect.w() as u32, wrect.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());

    let mut blob = Blob::new(wrect.w() as usize, wrect.h() as usize, rng, noise);
    blob.gen(rate, noise_scale, perlin_factor, scale, &colors);

    let color_settings = ColorSettings::new(2.0, Some((0.5, 1.5)), Some((1.2, 1.2)), Some(2.0));
    blob.renderer
        .render(srgba(0.1, 0.0, 0.0, 1.0), color_settings);

    Model {
        blob,
        texture,
        rate,
        scale,
        perlin_factor,
        noise_scale,
        colors,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let flat_samples = model.blob.renderer.img().as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        flat_samples.as_slice(),
    );

    let draw = app.draw();

    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
