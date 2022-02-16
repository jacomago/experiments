use std::fs::File;

use log_density::blob::Blob;
use log_density::renderer::ColorSettings;
use log_density::{basic_color, lerp_colors, BasicColor, PointParam};
use nannou::noise::{ Perlin};
use nannou::prelude::*;
use std::io::Write;

fn main() {
    nannou::app(model).run();
}

fn gen_point(
    xy: Vec2,
    noise: Perlin,
    point_param: &PointParam,
    colors: &[Srgba],
) -> (Vec2, BasicColor) {
    let r = point_param.noise_scale * xy.length();
    let t = vec2(xy.x.sin() * xy.y.cos() as f32, xy.y.cos() / xy.x);
    let nxy = r * t;
    (
        point_param.zero_point + point_param.scale * xy + nxy,
        basic_color(lerp_colors(colors, t.length())),
    )
}
fn expectation(xy: Vec2) -> f32 {
    1.0 - (-xy.length().pow(2.0) / 2.0).exp()
}
#[derive(Debug)]
struct Settings {
    color_settings: ColorSettings,
    point_param: PointParam,
    colors: Vec<Srgba>,
}

struct Model {
    blob: Blob,
    texture: wgpu::Texture,
    settings: Settings,
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let path = app
                .assets_path()
                .expect("Expected project path")
                .join("images")
                .join(app.exe_name().unwrap());
            app.main_window().capture_frame(
                path.join(format!("{:03}", app.elapsed_frames()))
                    .with_extension("png"),
            );
            let mut file = File::create(
                path.join(format!("{:03}", app.elapsed_frames()))
                    .with_extension("txt"),
            )
            .unwrap();
            writeln!(&mut file, "{:?}", model.settings).unwrap();
        }
        Key::G => {
            model.blob.gen(
                &model.settings.point_param,
                &model.settings.colors,
                gen_point,
                expectation,
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

    let noise = Perlin::new();

    let point_param = PointParam {
        scale: SIZE as f32 / 6.0,
        noise_scale: 75.0,
        zero_point: vec2(SIZE as f32 / 2.0, SIZE as f32 / 2.0),
        noise_pos: vec2(3.4, 5.6),
    };

    let colors = vec![
        srgba(0.76, 0.96, 0.9, 1.0),
        srgba(0.8, 0.0, 0.7, 1.0),
        srgba(0.2, 0.3, 0.5, 1.0),
    ];

    let wrect = app.window_rect();

    let window = app.main_window();
    let texture = wgpu::TextureBuilder::new()
        .size([wrect.w() as u32, wrect.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());

    let mut blob = Blob::new(wrect.w() as usize, wrect.h() as usize, noise);
    blob.gen(&point_param, &colors, gen_point, expectation);

    let color_settings = ColorSettings::new(2.0, Some((0.5, 1.5)), Some((1.5, 1.2)), Some(2.0));
    blob.renderer
        .render(srgba(0.1, 0.1, 0.1, 1.0), &color_settings);

    Model {
        blob,
        texture,
        settings: Settings {
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
