use nannou::{prelude::*, color::IntoLinSrgba};
use sorted_vec::SortedSet;

fn main() {
    nannou::app(model).update(update).run();
}
struct Spiral {
    step_size: f32,
    directions: Vec<Vec2>,
    pos: Vec2,
    ppos: Vec2,
    steps: u64,
    turns: u64,
    direction: usize,
    color:  Hsla
}

impl Spiral {
    fn new(sides: usize, size: usize, max: u64, color: Hsla) -> Self {
        let recip = (sides as f32).recip();
        let directions = (0..sides)
            .map(|x| {
                vec2(
                    (x as f32 * recip * TAU).sin(),
                    (x as f32 * recip * TAU).cos(),
                )
            })
            .collect();
        Self {
            step_size: size as f32 / (max as f32 + 1.0),
            directions,
            pos: Vec2::ZERO,
            ppos: Vec2::ZERO,
            turns: 1,
            steps: 1,
            direction: 0,
            color
        }
    }

    fn update(&mut self, n: u64) {
        self.ppos = self.pos;
        self.pos += self.step_size * self.directions[self.direction];
        if n % self.steps == 0 {
            self.turns += 1;
            self.direction = (self.direction + 1) % self.directions.len();
            if self.turns % 2 == 0 {
                self.steps += 1;
            }
        }
    }

    fn draw(&self, draw: &Draw, n: u64, prime: bool) {
        draw.line().points(self.ppos, self.pos).color(self.color);
        if prime {
            draw.ellipse().radius(3.0).xy(self.pos).color(self.color);
        }
    }
}

struct Model {
    field_up: f32,
    field_left: f32,
    primes: SortedSet<u64>,
    spiral: Spiral,
    max: u64,
    n: u64,
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

    let directions = vec![
        vec2(1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(-1.0, 0.0),
        vec2(0.0, -1.0),
    ];

    let max = 100;
    let n = 0;
    Model {
        primes: SortedSet::from(vec![2, 3, 5, 7, 11, 13, 17, 19]),
        field_up: 120.0,
        field_left: 1.0,
        spiral: Spiral::new(4, SIZE, max, WHITE.into()),
        max,
        n,
    }
}

fn prime(n: u64, primes: &mut SortedSet<u64>) {
    if n == 1 {
        return;
    }
    for p in primes.iter() {
        if *p as f64 > (n as f64).sqrt() {
            break;
        } else if n % p == 0 {
            return;
        }
    }
    primes.insert(n);
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let n = model.n;
    prime(n + 1, &mut model.primes);
    if n >= 1 && n < model.max.pow(2) {
        model.spiral.update(n);
    }
    model.n += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(LIGHTGREY);
    }

    let n = model.n;

    if n > model.max.pow(2) {
        return;
    }

    let prime = model.primes.contains(&n);
    model.spiral.draw(&draw, n, prime);

    draw.to_frame(app, &frame).unwrap();
}
