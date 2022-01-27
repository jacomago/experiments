use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model).run();
}

struct Fields {
    field: f64,
}

struct Model {
    noise: Perlin,
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

    let noise = Perlin::new();
    Model {
        noise,
        fields: Fields { field: 120.0 },
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    let rect = app.window_rect();

    let mut xoff = 0.0;
    for x in rect.left().floor() as i32..rect.right().floor() as i32 {
        let mut yoff = 0.0;
        for y in rect.left().floor() as i32..rect.right().floor() as i32 {
            let bright = map_range(model.noise.get([yoff, xoff]), -1.0, 1.0, 0.0, 1.0);
            draw.ellipse()
                .radius(1.0)
                .x_y(x as f32, y as f32)
                .color(hsl(0.5, 0.5, bright as f32));
            yoff += 0.01;
        }
        xoff += 0.1;
    }

    draw.to_frame(app, &frame).unwrap();
}
