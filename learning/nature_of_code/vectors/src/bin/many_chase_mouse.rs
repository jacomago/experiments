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

struct Ball {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
}

impl Ball {
    fn update(&mut self, mouse: Point2) {
        self.acceleration = (mouse - self.position).normalize_or_zero() * 0.2;
        self.velocity += self.acceleration;

        self.velocity = self.velocity.clamp_length_max(self.top_speed);

        self.position += self.velocity;
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(5.0)
            .color(STEELBLUE);
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

    let balls = (0..20)
        .map(|_| Ball {
            position: pt2(
                random::<f32>() * app.window_rect().w(),
                random::<f32>() * app.window_rect().h(),
            ),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            top_speed: 3.0,
        })
        .collect();
    Model {
        balls,
        fields: Fields { field: 120.0 },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for ball in model.balls.iter_mut() {
        ball.update(app.mouse.position());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    for ball in &model.balls {
        ball.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
