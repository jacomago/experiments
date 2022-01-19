use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Fields {
    radius: f32,
    x_speed: f32,
    y_speed: f32,
}

struct Thing {
    position: Vec2,
    color: Srgb,
}
struct Model {
    fields: Fields,
    things: Vec<Thing>,
}

const SIZE: usize = 500;
const N_THINGS: usize = 1000;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => refresh(model),
        Key::S => app
            .main_window()
            .capture_frame("images/".to_owned() + &app.exe_name().unwrap() + ".png"),
        Key::Up => model.fields.y_speed += 0.001,
        Key::Down => {
            if model.fields.y_speed > 0.0 {
                model.fields.y_speed -= 0.001;
            }
        }
        Key::Right => model.fields.x_speed += 1.0,
        Key::Left => {
            if model.fields.x_speed > 0.0 {
                model.fields.x_speed -= 0.1;
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

    let mut things = Vec::new();

    for _ in 1..N_THINGS {
        things.push(Thing {
            position: Vec2::new(
                (random::<f32>() - 0.5) * SIZE as f32,
                (random::<f32>() - 0.5) * SIZE as f32,
            ),
            color: *srgba(0.1, 0.1, 0.8, random()),
        });
    }
    Model {
        fields: Fields {
            radius: 1.0,
            x_speed: 0.01,
            y_speed: 0.01,
        },
        things,
    }
}

fn refresh(model: &mut Model) {
    
    let mut things = Vec::new();

    for _ in 1..N_THINGS {
        things.push(Thing {
            position: Vec2::new(
                (random::<f32>() - 0.5) * SIZE as f32,
                (random::<f32>() - 0.5) * SIZE as f32,
            ),
            color: *srgba(0.1, 0.1, 0.8, random()),
        });
    }
    model.things = things;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for thing in model.things.iter_mut() {
        thing.position -= vec2(
            thing.position.x * model.fields.x_speed,
            thing.position.y * model.fields.y_speed,
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(WHITE);
    }

    draw.rect()
        .w_h(SIZE as f32, SIZE as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in &model.things {
        draw.ellipse()
            .xy(thing.position)
            .radius(model.fields.radius)
            .color(thing.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
