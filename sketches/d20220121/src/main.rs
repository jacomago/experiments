use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Fields {
    init_speed: f32,
    n_balls: usize,
    radius: f32,
}

struct Model {
    balls: Vec<Ball>,
    fields: Fields,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.fields.init_speed += 1.0;
            model.balls = create_balls(model.fields.n_balls, model.fields.init_speed);
        }
        Key::S => app.main_window().capture_frame(format!(
            "{}/images/{app_name}.png",
            &app.exe_name().unwrap(),
            app_name = &app.exe_name().unwrap()
        )),
        Key::Up => {
            model.fields.n_balls += 1;
            model.balls.push(Ball::new(model.fields.init_speed));
        }
        Key::Down => {
            if model.fields.n_balls > 0 {
                model.fields.n_balls -= 1;
                model.balls.pop();
            }
        }
        Key::Right => model.fields.radius += 1.0,
        Key::Left => {
            if model.fields.radius > 0.0 {
                model.fields.radius -= 0.1;
            }
        }
        _other_key => {}
    }
}

struct WallBounce {
    left: bool,
    right: bool,
    bottom: bool,
    top: bool,
}

impl WallBounce {
    fn hue(&self) -> f32 {
        let increment = 1.0 / 16.0;
        0.0 + if self.left { increment } else { 0.0 }
            + 2.0 * if self.right { increment } else { 0.0 }
            + 4.0 * if self.bottom { increment } else { 0.0 }
            + 8.0 * if self.top { increment } else { 0.0 }
    }
}

struct Ball {
    position: Point2,
    speed: Vec2,
    color: Hsl,
    wall_bounce: WallBounce,
}

impl Ball {
    fn new(init_speed: f32) -> Self {
        let wall_bounce = WallBounce {
            left: random(),
            right: random(),
            top: random(),
            bottom: random(),
        };
        Ball {
            position: pt2(0.0, 0.0),
            speed: vec2(
                init_speed * (random::<f32>() - 0.5),
                init_speed * (random::<f32>() - 0.5),
            ),
            color: hsl(wall_bounce.hue(), 0.8, 0.5),
            wall_bounce,
        }
    }
    fn update(&mut self) {
        self.position += self.speed;
    }

    fn draw(&self, draw: &Draw, radius: f32) {
        draw.ellipse()
            .xy(self.position)
            .radius(radius)
            .color(self.color);
    }

    fn check_edges(&mut self, rect: &Rect) {
        if self.position.x < rect.left() {
            if self.wall_bounce.left {
                self.speed = -self.speed;
            } else {
                self.position.x = rect.right();
            }
        } else if self.position.x > rect.right() {
            if self.wall_bounce.right {
                self.speed = -self.speed;
            } else {
                self.position.x = rect.left();
            }
        }

        if self.position.y < rect.bottom() {
            if self.wall_bounce.bottom {
                self.speed = -self.speed;
            } else {
                self.position.y = rect.top();
            }
        } else if self.position.y > rect.top() {
            if self.wall_bounce.top {
                self.speed = -self.speed;
            } else {
                self.position.y = rect.bottom()
            }
        }
    }
}

fn create_balls(n_balls: usize, init_speed: f32) -> Vec<Ball> {
    let mut output = vec![];
    for _ in 0..n_balls {
        output.push(Ball::new(init_speed));
    }
    output
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

    let n_balls = 1000;
    let init_speed = 10.0;
    let balls = create_balls(n_balls, init_speed);
    Model {
        balls,
        fields: Fields {
            init_speed,
            n_balls,
            radius: 2.0,
        },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.balls.iter_mut().for_each(|ball| {
        ball.update();
        ball.check_edges(&app.window_rect());
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(LIGHTGREY);
    }

    draw.rect()
        .wh(app.window_rect().wh())
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for ball in &model.balls {
        ball.draw(&draw, model.fields.radius);
    }

    draw.to_frame(app, &frame).unwrap();
}
