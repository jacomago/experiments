use std::fs::File;

use interaction::save_path;
use log_density::blob::Blob;
use log_density::renderer::ColorSettings;
use log_density::{basic_color, lerp_colors, BasicColor, PointParam};

use nannou::noise::Perlin;
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::io::Write;

use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

fn gen_point(
    xy: Vec2,
    _noise: Perlin,
    point_param: &PointParam,
    colors: &[Srgba],
) -> (Vec2, BasicColor) {
    let r = point_param.noise_scale * xy.length();
    let psi = r * xy.angle();
    let t = xy.x
        * vec2(
            (psi + point_param.noise_pos.x).cos() + (psi + point_param.noise_pos.x).sin(),
            (psi + point_param.noise_pos.y).cos() - (psi + point_param.noise_pos.y).sin(),
        );
    let nxy = r * t;
    (
        point_param.zero_point + point_param.scale * xy + nxy,
        basic_color(lerp_colors(colors, t.length())),
    )
}

fn expectation(xy: Vec2) -> f32 {
    (-xy.length().sin().pow(2.0) / 2.0).exp()
}

#[derive(Debug)]
pub struct Settings {
    color_settings: ColorSettings,
    point_param: PointParam,
    colors: Vec<Srgba>,
    back_color: Srgba,
}

struct Model {
    main_window: WindowId,
    egui: Egui,
    blob: Blob,
    texture: wgpu::Texture,
    settings: Settings,
    updated: bool,
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let path = save_path(app);
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(
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
        }
        _other_key => {}
    }
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

const SIZE: usize = 1000;

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(280, 130)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window_ref = app.window(ui_window).unwrap();
    let egui = Egui::from_window(&ui_window_ref);

    let noise = Perlin::new();

    let point_param = PointParam {
        scale: SIZE as f32 / 6.0,
        noise_scale: 75.0,
        zero_point: vec2(SIZE as f32 / 2.0, SIZE as f32 / 2.0),
        noise_pos: vec2(3.4, 5.6),
    };

    let colors = vec![
        hsva(0.4, 0.5, 1.0, 1.0).into(),
        hsva(0.3, 0.7, 1.0, 1.0).into(),
        hsva(0.23, 0.6, 1.0, 1.0).into(),
    ];

    let wrect = app.window_rect();

    let window = app.main_window();
    let texture = wgpu::TextureBuilder::new()
        .size([wrect.w() as u32, wrect.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());

    let blob = Blob::new(wrect.w() as usize, wrect.h() as usize, noise);

    let color_settings = ColorSettings::new(2.0, Some((0.5, 1.5)), Some((1.5, 1.2)), Some(2.0));

    let back_color = hsv(0.01, 0.1, 0.01).into();
    Model {
        main_window,
        egui,
        blob,
        texture,
        settings: Settings {
            color_settings,
            point_param,
            colors,
            back_color,
        },
        updated: false,
    }
}

pub fn edit_color(ui: &mut egui::Ui, color: &mut Srgba) -> bool {
    let egui_srgba =
        egui::color::Rgba::from_rgba_premultiplied(color.red, color.green, color.blue, color.alpha);
    let mut egui_color32: egui::color::Color32 = egui_srgba.into();
    let mut changed = false;
    if egui::color_picker::color_edit_button_srgba(
        ui,
        &mut egui_color32,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        let egui_srgba: egui::color::Rgba = egui_color32.into();
        *color = nannou::color::srgba(
            egui_srgba.r(),
            egui_srgba.g(),
            egui_srgba.b(),
            egui_srgba.a(),
        );
        changed = true
    }
    changed
}

fn egui_update(ui: &mut Ui, settings: &mut Settings) -> bool {
    ui.add(egui::Label::new("rendered movement"));
    let mut changed = false;
    changed = changed || edit_color(ui, &mut settings.back_color);
    changed = changed || settings.color_settings.ui_update(ui);
    changed = changed || settings.point_param.ui_update(ui);

    for c in &mut settings.colors {
        changed = changed || edit_color(ui, c);
    }
    changed
}

fn update_ui(egui: &mut Egui, settings: &mut Settings) -> bool {
    let ctx = egui.begin_frame();
    let mut changed = false;
    egui::Window::new("Workshop window").show(&ctx, |ui| {
        //vels
        changed = egui_update(ui, settings);
    });
    changed
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut blob,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let changed = update_ui(egui, settings);
    if changed || app.elapsed_frames() == 0 {
        blob.gen(
            &settings.point_param,
            &settings.colors,
            gen_point,
            expectation,
        );
        blob.renderer
            .render(settings.back_color, &settings.color_settings);
        model.updated = true;
    } else {
        model.updated = false;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if model.updated {
        frame.clear(BLACK);

        let flat_samples = model.blob.renderer.img().as_flat_samples();
        model.texture.upload_data(
            app.main_window().device(),
            &mut *frame.command_encoder(),
            flat_samples.as_slice(),
        );

        draw.texture(&model.texture);
    }

    draw.to_frame(app, &frame).unwrap();
}
