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

struct Model {
    balls: Vec<Ball>,
    force: Perlin,
    field_up: f32,
    field_left: f32,
    boundary: f32,
}

struct Ball {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    color: Hsla,
}

impl Ball {
    fn new(mass: f32, color: Hsla) -> Self {
        let r = (random::<f32>() - 0.5) * TAU;
        Ball {
            position: pt2(SIZE as f32 * 0.5 * r.cos(), SIZE as f32 * 0.5 * r.sin()),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            mass,
            color,
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
            .color(self.color);
    }

    fn boundaries(&mut self, rect: Rect, force_boundry: f32) {
        let d = rect.wh() * force_boundry;
        let mut force = vec2(0.0, 0.0);

        if self.position.x < rect.pad(d.x).left() {
            force.x = 1.0;
        } else if self.position.x > rect.pad(d.x).right() {
            force.x = -1.0;
        }
        if self.position.y > rect.pad(d.y).top() {
            force.y = -1.0;
        } else if self.position.y < rect.pad(d.y).bottom() {
            force.y = 1.0;
        }
        if force.length() > 0.0 {
            force = force.normalize();
            force *= 0.65;
            self.apply_force(force);
        }
    }
}
const SIZE: usize = 1024;
const COUNT: usize = 200;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);
    match key {
        Key::W => model.boundary -= 0.01,
        Key::E => model.boundary += 0.01,
        _other_key => {}
    }
}

fn ball_size(x: usize, count: usize) -> f32 {
    map_range(x, 0, count, 1.0, 6.0)
}

fn ball_color(x: usize, count: usize, start: f32, step: f32) -> Hsla {
    hsla(map_range(x, 0, count, start, start + step), 1.0, 0.7, 1.0)
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

    let mut balls: Vec<Ball> = (0..COUNT)
        .map(|x| Ball::new(ball_size(x, COUNT), ball_color(x, COUNT, 0.2, 0.2)))
        .collect();

    let mut new_balls: Vec<Ball> = (0..COUNT)
        .map(|x| Ball::new(ball_size(x, COUNT), ball_color(x, COUNT, 0.4, 0.2)))
        .collect();

    balls.append(&mut new_balls);

    let mut new_balls: Vec<Ball> = (0..COUNT)
        .map(|x| Ball::new(ball_size(x, COUNT), ball_color(x, COUNT, 0.6, 0.2)))
        .collect();

    balls.append(&mut new_balls);
    let force = Perlin::new();

    Model {
        balls,
        force,
        field_up: 0.08,
        field_left: 0.05,
        boundary: 0.5,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = model.field_up as f64 * app.elapsed_frames() as f64;
    for ball in model.balls.iter_mut() {
        let force = model.field_left
            * vec2(
                model.force.get([ball.position.x as f64, t]) as f32,
                model.force.get([ball.position.y as f64, t]) as f32,
            );
        ball.apply_force(force);
        ball.update();
        ball.boundaries(app.window_rect(), model.boundary);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .wh(app.window_rect().wh())
        .color(srgba(0.8, 0.8, 0.8, 0.05));

    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
