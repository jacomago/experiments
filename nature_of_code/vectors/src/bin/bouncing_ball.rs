use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Fields {
    field: f64,
}

struct Model {
    position: Point2,
    speed: Vec2,
    fields: Fields,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}/images/{app_name}.png",
            &app.exe_name().unwrap(),
            app_name = &app.exe_name().unwrap()
        )),
        Key::Up => model.fields.field += 0.001,
        Key::Down => {
            if model.fields.field > 0.0 {
                model.fields.field -= 0.001;
            }
        }
        Key::Right => model.fields.field += 1.0,
        Key::Left => {
            if model.fields.field > 0.0 {
                model.fields.field -= 0.1;
            }
        }
        _other_key => {}
    }
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
        position: pt2(0.0, 0.0),
        speed: vec2(1.0, 3.3),
        fields: Fields { field: 120.0 },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.position += model.speed;

    let rect = app.window_rect();
    if model.position.x > rect.right() || model.position.x < rect.left() {
        model.speed.x *= -1.0;
    }
    if model.position.y > rect.top() || model.position.y < rect.bottom() {
        model.speed.y *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    draw.ellipse().xy(model.position).color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
