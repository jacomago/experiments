use nannou::prelude::*;
use sorted_vec::SortedSet;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    field_up: f32,
    field_left: f32,
    primes: SortedSet<u64>,
    step_size: f32,
    directions: Vec<Vec2>,
    pos: Vec2,
    ppos: Vec2,
    steps: u64,
    turns: u64,
    direction: usize,
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
        step_size: SIZE as f32 / (max as f32 + 1.0),
        directions,
        pos: Vec2::ZERO,
        ppos: Vec2::ZERO,
        turns: 1,
        steps: 1,
        direction: 0,
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
        model.ppos = model.pos;
        model.pos += model.step_size * model.directions[model.direction];
        if n % model.steps == 0 {
            model.turns += 1;
            model.direction = (model.direction + 1) % model.directions.len();
            if model.turns % 2 == 0 {
                model.steps += 1;
            }
        }
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

    draw.line().points(model.ppos, model.pos).color(WHITE);

    if model.primes.contains(&n) {
        draw.ellipse().radius(3.0).xy(model.pos).color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
