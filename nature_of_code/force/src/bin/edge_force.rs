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
    fn new(mass: f32) -> Self {
        Ball {
            position: pt2(SIZE as f32 * 0.1 * random::<f32>(), SIZE as f32 * 0.1 * random::<f32>()),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.001, 0.0),
            mass,
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
            .color(hsl(
                0.5,
                0.5,
                map_range(self.mass, 1.0, SIZE as f32 / 2.0, 0.0, 1.0),
            ));
    }

    fn check_edges(&mut self, rect: Rect) {
        let force = if (rect.left()..(rect.right() - rect.w() * 0.5)).contains(&self.position.x) {
            vec2(0.1 / (self.position.x - rect.left()), 0.0)
        } else {
            vec2(0.1 / (self.position.x - rect.right()), 0.0)
        };
        self.apply_force(force);
        if self.position.y > rect.top() || self.position.y < rect.bottom() {
            self.velocity.y *= -1.0;
        }
    }
}

struct Model {
    balls: Vec<Ball>,
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

    let balls = (0..20).map(|x: i32| Ball::new(x as f32)).collect();

    Model {
        balls,
        gravity: vec2(0.0, -0.05),
        wind: vec2(0.01, 0.0),
        noise: Perlin::new(),
        fields: Fields { field: 120.0 },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames();
    model.wind = 0.5
        * vec2(
            map_range(
                model.noise.get([t as f64 * 0.01, 1.0]),
                -1.0,
                1.0,
                -0.1,
                0.1,
            ),
            0.0,
        );
    for ball in model.balls.iter_mut() {
        ball.apply_force(model.gravity);
        ball.apply_force(model.wind);
        ball.update();
        ball.check_edges(app.window_rect());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
