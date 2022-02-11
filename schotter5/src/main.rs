use std::{fs, io::ErrorKind, path::PathBuf};

use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};
use nannou_egui::{self, egui, Egui};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
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
        Key::Up => model.disp_adj += 0.1,
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => model.rot_adj += 0.1,
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }
}

struct StoneNoise {
    x: NoiseLoop<f32>,
    y: NoiseLoop<f32>,
    rot: NoiseLoop<f32>,
    motion: NoiseLoop<f32>,
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
    noise: StoneNoise,
}

impl Stone {
    fn new(x: f32, y: f32, diameter: f64) -> Self {
        let noise = StoneNoise {
            x: NoiseLoop::new(diameter, -0.5, 0.5),
            y: NoiseLoop::new(diameter, -0.5, 0.5),
            rot: NoiseLoop::new(diameter, -PI / 2.0, PI / 2.0),
            motion: NoiseLoop::new(diameter, 0.0, 1.0),
        };
        Stone {
            x,
            y,
            x_offset: 0.0,
            y_offset: 0.0,
            rotation: 0.0,
            noise,
        }
    }
}

struct Model {
    ui: Egui,
    main_window: WindowId,
    disp_adj: f32,
    rot_adj: f32,
    motion: f32,
    time_factor: f32,
    gravel: Vec<Stone>,
    frames_dir: PathBuf,
    cur_frame: u32,
    noise: Perlin,
    recording: bool,
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
            ui.add(egui::Slider::new(&mut model.motion, 0.0..=1.0).text("Motion"));
            ui.add(egui::Slider::new(&mut model.time_factor, 0.0..=0.01).text("Time Factor"));
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

    let disp_adj = 1.0;
    let rot_adj = 1.0;

    let period_length = 50.0;
    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            let stone = Stone::new(x as f32, y as f32, period_length);
            gravel.push(stone);
        }
    }

    let motion = 0.5;
    let time_factor = 0.01;
    let frames_dir = app
        .assets_path()
        .expect("Expected project path")
        .join("images")
        .join("gif")
        .join("output")
        .join(app.exe_name().unwrap());
    let recording = false;
    let cur_frame = 0;

    let noise = Perlin::new();
    Model {
        ui,
        main_window,
        disp_adj,
        rot_adj,
        motion,
        time_factor,
        gravel,
        frames_dir,
        cur_frame,
        noise,
        recording,
    }
}

struct NoiseLoop<T>
where
    T: NumCast + Copy,
{
    diameter: f64,
    min: T,
    max: T,
    seed: (f64, f64),
}

impl<T: NumCast + Copy> NoiseLoop<T> {
    fn new(diameter: f64, min: T, max: T) -> Self {
        let seed = (1000.0 * random::<f64>(), 1000.0 * random::<f64>());
        NoiseLoop {
            diameter,
            min,
            max,
            seed,
        }
    }

    fn value(&self, a: f32, noise: Perlin) -> T {
        let x = map_range(a.cos(), -1.0, 1.0, self.seed.0, self.seed.0 + self.diameter);
        let y = map_range(a.sin(), -1.0, 1.0, self.seed.1, self.seed.1 + self.diameter);
        let r = noise.get([x, y, 0.0]);
        map_range(r, 0.0, 1.0, self.min, self.max)
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    update_ui(model);

    let t = app.elapsed_frames();
    let tf = TAU * model.time_factor * t as f32;

    for stone in &mut model.gravel {
        if stone.noise.motion.value(tf, model.noise) < model.motion {
            let factor = stone.y / ROWS as f32;
            let disp_factor = factor * model.disp_adj;

            stone.x_offset = disp_factor * stone.noise.x.value(tf, model.noise);
            stone.y_offset = disp_factor * stone.noise.y.value(tf, model.noise);

            let rot_factor = factor * model.rot_adj;
            stone.rotation = rot_factor * stone.noise.rot.value(tf, model.noise);
        }
    }

    if model.recording {
        model.cur_frame += 1;
        if model.cur_frame as f32 > (1.0 / model.time_factor) {
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

    for stone in &model.gravel {
        let cdraw = gdraw.x_y(stone.x, stone.y);
        cdraw
            .rect()
            .no_fill()
            .stroke(STEELBLUE)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.rotation);
    }
    draw.to_frame(app, &frame).unwrap();
}
