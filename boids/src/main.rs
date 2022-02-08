use boids::flock::Flock;
use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

use nannou_egui::{self, egui, Egui};

// TODO: Better visuals
// TODO: SpatialSplit calc

// TODO: 3D???

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    main_window: WindowId,
    egui: Egui,
    flocks: Vec<Flock>,
    wind_force: Vec2,
    noise: Perlin,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let mut y_force = model.wind_force.y;
    interaction::key_pressed(app, &mut y_force, &mut model.wind_force.x, key);
    model.wind_force.y = y_force;
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
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

    let flocks = vec![
        Flock::new("Seagulls".to_string(), hsv(1.0, 1.0, 1.0)),
        Flock::new("Crows".to_string(), hsv(1.0, 0.8, 0.8)),
        Flock::new("Sparrows".to_string(), hsv(0.8, 0.5, 1.0)),
    ];

    Model {
        main_window,
        egui,
        flocks,
        wind_force: vec2(0.0, 0.0),
        noise: Perlin::new(),
    }
}

fn update_ui(egui: &mut Egui, flocks: &mut Vec<Flock>) {
    let ctx = egui.begin_frame();
    egui::Window::new("Workshop window").show(&ctx, |ui| {
        //vels
        for flock in flocks {
            flock.egui_update(ui);
        }
    });
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut flocks,
        ref wind_force,
        ref noise,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    update_ui(egui, flocks);

    let t = app.elapsed_frames() as f64 * 0.1;
    let wind_force_acc = *wind_force * noise.get([t.cos(), t.sin(), 0.0]) as f32;

    let rect = if let Some(window) = app.window(model.main_window) {
        window.rect()
    } else {
        app.window_rect()
    };

    for flock in flocks.iter_mut() {
        flock.update(rect, wind_force_acc);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for flock in &model.flocks {
        flock.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
