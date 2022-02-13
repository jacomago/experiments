use std::{fs, io::ErrorKind, path::PathBuf};

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use schotter5::gravel::Gravel;

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            if model.recording {
                model.recording = false;
            } else {
                fs::create_dir(&model.frames_dir).unwrap_or_else(|error| {
                    if error.kind() != ErrorKind::AlreadyExists {
                        panic! {"Problem creating directory {:?}", model.frames_dir};
                    }
                });
                model.recording = true;
                model.cur_frame = 0;
            }
        }
        Key::S => {
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(
                    app.assets_path()
                        .expect("Expected project path")
                        .join("images")
                        .join(app.exe_name().unwrap())
                        .join(format!("{:03}", app.elapsed_frames()))
                        .with_extension("png"),
                )
            }
        }
        _other_key => {}
    }
}

struct Model {
    ui: Egui,
    main_window: WindowId,
    gravel: Gravel,
    frames_dir: PathBuf,
    cur_frame: u32,
    recording: bool,
    period_length: u32,
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            model.gravel.update_ui(ui);
        });
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
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
    let ui = Egui::from_window(&ui_window_ref);

    let period_length = 300;

    let gravel = Gravel::new(ROWS, COLS, period_length);

    let frames_dir = app
        .assets_path()
        .expect("Expected project path")
        .join("images")
        .join("gif")
        .join("output")
        .join(app.exe_name().unwrap());
    let recording = false;
    let cur_frame = 0;

    Model {
        ui,
        main_window,
        gravel,
        frames_dir,
        cur_frame,
        recording,
        period_length,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    model.gravel.update();
    if model.recording {
        model.cur_frame += 1;
        if model.cur_frame > 2 * model.period_length {
            model.recording = false;
        } else {
            let filename = model
                .frames_dir
                .join(format!("schotter{:>04}", model.cur_frame))
                .with_extension("png");

            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(filename);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(PLUM);
    model.gravel.draw(&gdraw);
    draw.to_frame(app, &frame).unwrap();
}
