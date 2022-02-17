use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Fields {
    max_size: f32,
}

struct Model {
    position: Vec2,
    fields: Fields,
    new_position: Vec2,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}/images/{app_name}.png",
            &app.exe_name().unwrap(),
            app_name = &app.exe_name().unwrap()
        )),
        Key::Up => model.fields.max_size += 0.001,
        Key::Down => {
            if model.fields.max_size > 0.0 {
                model.fields.max_size -= 0.001;
            }
        }
        Key::Right => model.fields.max_size += 1.0,
        Key::Left => {
            if model.fields.max_size > 0.0 {
                model.fields.max_size -= 0.1;
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
        position: vec2(0.0, 0.0),
        new_position: vec2(0.0, 0.0),
        fields: Fields { max_size: 50.0 },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.position = model.new_position;
    let size: f32 = (model.fields.max_size.sqrt() * random::<f32>()).powi(2);
    let add = size * vec2(random::<f32>() - 0.5, random::<f32>() - 0.5);
    model.new_position += add;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }
    draw.point_mode();
    draw.line()
        .start(model.position)
        .end(model.new_position)
        .weight(1.0)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
