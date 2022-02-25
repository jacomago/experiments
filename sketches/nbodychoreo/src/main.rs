use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

#[derive(Debug, Copy, Clone)]
struct Thing {
    mass: f32,
    pos: Vec2,
    velocity: Vec2,
    acc: Vec2,
}

impl Thing {
    fn update(&mut self, rect: &Rect, top_speed: f32) {
        self.velocity += self.acc;
        self.pos += self.velocity;
        self.velocity = self.velocity.clamp_length_max(top_speed);
        self.check_edges(rect);
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse().radius(self.mass).xy(self.pos).color(WHITE);
    }

    fn add_gravity(&mut self, thing: &Thing, grav: f32) {
        let dir = (thing.pos - self.pos).clamp_length(5.0, 25.0);
        let recip = self.mass * thing.mass * dir.length_squared().recip();
        self.acc += grav * dir.normalize() * recip;
    }

    pub fn check_edges(&mut self, rect: &Rect) {
        if self.pos.x < rect.left() {
            self.pos.x = rect.right();
        } else if self.pos.x > rect.right() {
            self.pos.x = rect.left()
        }

        if self.pos.y < rect.bottom() {
            self.pos.y = rect.top();
        } else if self.pos.y > rect.top() {
            self.pos.y = rect.bottom()
        }
    }
}

struct System {
    things: Vec<Thing>,
    grav: f32,
    top_speed: f32,
}

impl System {
    fn new(starting_position: StartingPosition, top_speed: f32, grav: f32) -> Self {
        let things = starting_position
            .pairs
            .iter()
            .map(|p| Thing {
                mass: p.0.x.clamp(1.0, 20.0),
                pos: p.0,
                velocity: p.1,
                acc: Vec2::ZERO,
            })
            .collect();
        Self {
            things,
            grav,
            top_speed,
        }
    }

    fn update(&mut self, rect: &Rect) {
        let copy = self.things.clone();
        for t in self.things.iter_mut() {
            for thing in &copy {
                if (thing.pos - t.pos).length_squared() > f32::EPSILON {
                    t.add_gravity(thing, self.grav);
                }
            }
            t.update(rect, self.top_speed);
        }
    }

    fn draw(&self, draw: &Draw) {
        self.things.iter().for_each(|t| t.draw(draw));
    }
}

#[derive(Debug)]
struct StartingPosition {
    pairs: Vec<(Vec2, Vec2)>,
}

impl StartingPosition {
    fn circle(n: usize, radius: f32, speed: f32) -> Self {
        let pairs = (0..n)
            .map(|i| {
                let theta = TAU * (i as f32) * (n as f32).recip();
                let pos = radius * vec2(theta.cos(), theta.sin());
                let velocity = if pos.y > f32::EPSILON {
                    speed * vec2(1.0, -pos.x / pos.y).normalize()
                } else {
                    speed * vec2(pos.y / pos.x, -1.0).normalize()
                };
                (pos, velocity)
            })
            .collect();
        Self { pairs }
    }
}

struct Model {
    system: System,
    field_up: f32,
    field_left: f32,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);
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

    let position = StartingPosition::circle(10, 50.0, 1.0);
    let system = System::new(position, 10.0, 2.0);

    Model {
        system,
        field_up: 120.0,
        field_left: 1.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.system.update(&app.window_rect());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .wh(app.window_rect().wh())
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    model.system.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
