use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Fields {
    field: f64,
}

struct Ball {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Ball {
    fn new() -> Self {
        Ball {
            position: pt2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            mass: 5.0,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.mass)
            .color(GREY);
    }

    fn check_edges(&mut self, rect: Rect) {
        if self.position.x > rect.right() || self.position.x < rect.left() {
            self.velocity.x *= -1.0;
        }
        if self.position.y > rect.top() || self.position.y < rect.bottom() {
            self.velocity.y *= -1.0;
        }
    }
}

struct Model {
    ball: Ball,
    gravity: Vec2,
    wind: Vec2,
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

    Model {
        ball: Ball::new(),
        gravity: vec2(0.0, 0.05),
        wind: vec2(0.01, 0.0),
        noise: Perlin::new(),
        fields: Fields { field: 120.0 },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames();
    model.wind = vec2(
        map_range(
            model.noise.get([t as f64 * 0.01, 1.0]),
            -1.0,
            1.0,
            -0.1,
            0.1,
        ),
        0.0,
    );
    model.ball.apply_force(model.wind);
    model.ball.apply_force(model.gravity);
    model.ball.update();
    model.ball.check_edges(app.window_rect());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    model.ball.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
