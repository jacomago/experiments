use nannou::{
    prelude::*,
    rand::{thread_rng, Rng},
};
use rand_distr::StandardNormal;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::wait())
        .update(update)
        .run();
}

struct Fields {
    color_ratio: f32,
    number_of_balls: i32,
    position_ratio: f32,
    radius_ratio: f32,
}

struct Model {
    balls: Vec<Ball>,
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
        Key::Up => model.fields.color_ratio += 0.001,
        Key::Down => {
            if model.fields.color_ratio > 0.0 {
                model.fields.color_ratio -= 0.001;
            }
        }
        Key::Right => model.fields.position_ratio += 1.0,
        Key::Left => {
            if model.fields.position_ratio > 0.0 {
                model.fields.position_ratio -= 0.1;
            }
        }
        Key::PageUp => model.fields.radius_ratio += 1.0,
        Key::PageDown => {
            if model.fields.radius_ratio > 0.0 {
                model.fields.radius_ratio -= 0.1;
            }
        }
        Key::Comma => model.fields.number_of_balls += 1,
        Key::Stop => {
            if model.fields.number_of_balls > 0 {
                model.fields.number_of_balls -= 1;
            }
        }
        _other_key => {}
    }
}

struct Ball {
    position: Vec2,
    color: Srgb,
    radius: f32,
}

impl Ball {
    fn new(position_ratio: f32, color_ratio: f32, radius_ratio: f32) -> Self {
        Ball {
            position: position_ratio * vec2(rand(), rand()),
            color: srgb(
                rand() * color_ratio,
                rand() * color_ratio,
                rand() * color_ratio,
            ),
            radius: radius_ratio * rand().abs(),
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
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

    let number_of_balls = 500;
    let color_ratio = 0.5;
    let position_ratio = SIZE as f32 * 0.1;
    let radius_ratio = 3.0;
    let balls = (0..number_of_balls)
        .map(|_| Ball::new(position_ratio, color_ratio, radius_ratio))
        .collect();

    Model {
        balls,
        fields: Fields {
            color_ratio,
            number_of_balls,
            position_ratio,
            radius_ratio,
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.balls = (0..model.fields.number_of_balls)
        .map(|_| {
            Ball::new(
                model.fields.position_ratio,
                model.fields.color_ratio,
                model.fields.radius_ratio,
            )
        })
        .collect();
}

fn rand() -> f32 {
    thread_rng().sample(StandardNormal)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
