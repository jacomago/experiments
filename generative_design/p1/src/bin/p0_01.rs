use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
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
    let mouse = app.mouse.position();
    let rect = app.window_rect();
    draw.background().hsl(
        map_range(mouse.y, rect.bottom(), rect.top(), 0.0, 1.0),
        1.0,
        0.5,
    );
    draw.rect().w(2.0 * mouse.x).h(2.0 * mouse.x).hsv(
        1.0 - map_range(mouse.y, rect.left(), rect.right(), 0.0, 1.0),
        1.0,
        0.5,
    );

    draw.to_frame(app, &frame).unwrap();
}
