use std::fs::File;

use log_density::lerp_colors;
use log_density::renderer::{ColorSettings, Renderer};
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou::rand::prelude::ThreadRng;
use nannou::rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use std::io::Write;

fn main() {
    nannou::app(model).run();
}

struct Blob {
    renderer: Renderer,
    rng: ThreadRng,
    noise: Perlin,
}
#[derive(Debug)]
struct PointParam {
    zero_point: Vec2,
    noise_pos: Vec2,
    noise_scale: f32,
    scale: f32,
}

fn gen_point(
    rng: &mut ThreadRng,
    noise: Perlin,
    point_param: &PointParam,
    colors: &[Srgba],
) -> (Vec2, Srgba) {
    let xy = vec2(rng.sample(StandardNormal), rng.sample(StandardNormal));
    let r = point_param.noise_scale * xy.length();
    let t = vec2(
        (noise.get([xy.x as f64, xy.y as f64, point_param.noise_pos.x.into()]) - 0.5) as f32,
        (noise.get([
            (xy.y - 1.1) as f64,
            (xy.x + 1.1) as f64,
            point_param.noise_pos.y.into(),
        ]) - 0.5) as f32,
    );
    let nxy = r * t;
    (
        point_param.zero_point + point_param.scale * xy + nxy,
        lerp_colors(colors, t.length()),
    )
}

#[derive(Debug)]
struct Settings {
    color_settings: ColorSettings,
    rate: usize,
    point_param: PointParam,
    colors: Vec<Srgba>,
}
struct Model {
    blob: Blob,
    texture: wgpu::Texture,
    settings: Settings,
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

    fn gen(&mut self, rate: usize, point_param: &PointParam, colors: &[Srgba]) {
        (0..rate).for_each(|_i| {
            let (point, color) = gen_point(&mut self.rng, self.noise, point_param, colors);
            self.renderer.add(point, color)
        });
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let path = app
                .assets_path()
                .expect("Expected project path")
                .join("images")
                .join(app.exe_name().unwrap());
            app.main_window().capture_frame(path
                .join(format!("{:03}", app.elapsed_frames()))
                .with_extension("png"));
            let mut file = File::create(path
                    .join(format!("{:03}", app.elapsed_frames()))
                    .with_extension("txt"),
            )
            .unwrap();
            writeln!(&mut file, "{:?}", model.settings).unwrap();
        }
        Key::G => {
            model.blob.gen(
                model.settings.rate,
                &model.settings.point_param,
                &model.settings.colors,
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

    let rate = 90000000;
    let point_param = PointParam {
        scale: SIZE as f32 / 6.0,
        noise_scale: 75.0,
        zero_point: vec2(SIZE as f32 / 2.0, SIZE as f32 / 2.0),
        noise_pos: vec2(3.4, 5.6),
    };

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
    blob.gen(rate, &point_param, &colors);

    let color_settings = ColorSettings::new(2.0, Some((0.5, 1.5)), Some((1.2, 1.2)), Some(2.0));
    blob.renderer
        .render(srgba(0.1, 0.0, 0.0, 1.0), &color_settings);

    Model {
        blob,
        texture,
        settings: Settings {
            rate,
            color_settings,
            point_param,
            colors,
        },
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
