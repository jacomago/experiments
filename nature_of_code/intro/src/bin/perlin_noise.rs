use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model).loop_mode(LoopMode::wait()).run();
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
    let t = app.elapsed_frames();
    if t % 10 == 0 {
        let rect = app.window_rect();
        draw.background().color(WHITE);
        let toff = t as f64 * 0.01;
        let mut xoff = 0.0;
        let step = 5;
        for x in (rect.left().floor() as i32..rect.right().floor() as i32).step_by(step) {
            let mut yoff = 0.0;
            for y in (rect.left().floor() as i32..rect.right().floor() as i32).step_by(step) {
                let bright = map_range(model.noise.get([yoff, xoff, toff]), -1.0, 1.0, 0.0, 1.0);
                let hue = map_range(model.noise.get([xoff, yoff, toff]), -1.0, 1.0, 0.1, 0.2);
                draw.quad()
                    .w_h(step as f32, step as f32)
                    .x_y(x as f32, y as f32)
                    .color(hsl(hue, 0.5, bright as f32));
                yoff += 0.05
            }
            xoff += 0.05;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
