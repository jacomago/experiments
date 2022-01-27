use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model).update(update).run();
}

struct Fields {
    field: f64,
}

struct Walker {
    position: Vec2,
    t: Vec2,
    color: Srgb<u8>,
}

struct Model {
    walkers: Vec<Walker>,
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
    let walkers = vec![
        Walker {
            position: vec2(0.0, 0.0),
            t: vec2(0.0, 10000.0),
            color: BLACK,
        },
        Walker {
            position: vec2(0.0, 0.0),
            t: vec2(0.0, 10000.0),
            color: RED,
        },
        Walker {
            position: vec2(0.0, 0.0),
            t: vec2(0.0, 10000.0),
            color: BLUE,
        },
        Walker {
            position: vec2(0.0, 0.0),
            t: vec2(0.0, 10000.0),
            color: YELLOW,
        },
    ];
    Model {
        walkers,
        noise,
        fields: Fields { field: 120.0 },
    }
}

impl Walker {
    fn update(&mut self, noise: Perlin) {
        let xy = vec2(
            noise.get([self.t.x as f64, 0.0]) as f32,
            noise.get([self.t.y as f64, 0.0]) as f32,
        );
        if self.color == BLACK {
            self.position = xy * SIZE as f32;
        } else if self.color == RED {
            self.position += (xy - vec2(5.0, 5.0)) * 0.1 * SIZE as f32;
        } else if self.color == BLUE {
            self.position = SIZE as f32 * (xy - 1.0) *0.5;
        } else {
            self.position += xy.x * 0.1* SIZE as f32 * vec2(random(), random());
        }
        self.t += vec2(0.01, 0.01);
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(1.0)
            .color(self.color);
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for walker in model.walkers.iter_mut() {
        walker.update(model.noise);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }
    for walker in &model.walkers {
        walker.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
