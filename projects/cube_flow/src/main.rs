use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    field_up: f32,
    field_left: f32,
    size: f32,
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
        size: 10.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let t = app.elapsed_frames() as f32 * 0.1;
    let rect = app.window_rect();
    draw.background().color(PLUM);
    let w = rect.w() * 0.09;
    let margin = rect.w() * 0.01;
    for x in 0..10 {
        draw.rect()
            .x(rect.left() + (w + margin) * (x as f32 + 0.5))
            .w(w)
            .h(20.0 + 50.0 * (t + x as f32 * 0.1).sin())
            .color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}
