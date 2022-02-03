use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Bloom {
    iterations: usize,
    resolution: usize,
    intensity: f32,
    threshold: f32,
    soft_knee: f32,
}

struct Sunrays {
    resolution: usize,
    weight: usize,
}

struct Config {
    sim_resolution: usize,
    dye_resolution: usize,
    capture_resolution: usize,
    density_dissipation: f32,
    velocity_dissipation: f32,
    pressure: f32,
    pressure_iterations: usize,
    curl: usize,
    splat_radius: f32,
    splat_force: usize,
    shading: bool,
    colorful: bool,
    color_update_speed: usize,
    paused: bool,
    back_color: Srgb,
    transparent: bool,
    bloom: Option<Bloom>,
    sunrays: Option<Sunrays>,
}

struct Model {
    config: Config,
    field_up: f32,
    field_left: f32,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        field_up: 120.0,
        field_left: 1.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
