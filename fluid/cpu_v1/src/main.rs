use cpu_v1::fluid_object::FluidCube;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    field: FluidCube,
    field_up: f32,
    field_left: f32,
    mouse_event: (Vec2, usize),
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_event = (
        app.mouse.position(),
        app.elapsed_frames().try_into().unwrap(),
    );
    model
        .field
        .add_density(app.mouse.position(), 1.0, app.window_rect());
}

fn mouse_released(app: &App, model: &mut Model, _button: MouseButton) {
    let v = app.mouse.position() - model.mouse_event.0;
    let v = v.normalize();
    model
        .field
        .add_velocity(model.mouse_event.0, v, app.window_rect());
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_released(mouse_released)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let scale = 0.5;
    let rect = app.window_rect();
    let wh = scale * rect.wh();

    let mut field = FluidCube::new(
        (wh.x.floor() as usize, wh.y.floor() as usize),
        0.5,
        1.0,
        1.0,
        4,
    );
    for x in rect.left().floor() as usize..rect.left().floor() as usize + SIZE / 2 {
        for y in rect.bottom().floor() as usize..rect.bottom().floor() as usize + SIZE / 2 {
            let pos = vec2(x as f32, y as f32);
            if random() {
            field.add_velocity(pos, vec2(random::<f32>(), random::<f32>()), rect);
            }
            if random() {
            field.add_density(pos, random::<f32>(), rect);
            }
        }
    }
    Model {
        field,
        field_up: 120.0,
        field_left: 1.0,
        mouse_event: (Vec2::ZERO, 0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.field.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    model.field.draw(&draw, app.window_rect(), true);

    draw.to_frame(app, &frame).unwrap();
}
