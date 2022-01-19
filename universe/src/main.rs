use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

// tutorial from https://www.youtube.com/watch?v=Ml6tpyTyXhM&t=776s
// @Mactuitui

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let positions = vec![p];
        Thing { positions }
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
    _window: window::Id,
}

const N_THINGS: usize = 500;
const SIZE: usize = 300;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .unwrap();

    let mut things = Vec::new();
    for _ in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * SIZE as f32,
            (random::<f32>() - 0.5) * SIZE as f32,
        ));
        things.push(thing);
    }
    let noise = Perlin::new();
    Model {
        things,
        noise,
        _window,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f64 / 120.0;
    let sn = 0.01 * time.cos() as f64 * 0.005;
    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * SIZE as f32,
            (random::<f32>() - 0.5) * SIZE as f32,
        ));

        for _ in 0..50 {
            let last_position = thing.positions[0];
            thing.positions.insert(
                0,
                last_position
                    + vec2(
                        model.noise.get([
                            sn * last_position.x as f64,
                            sn * last_position.y as f64,
                            0.0,
                        ]) as f32,
                        model.noise.get([
                            sn * last_position.x as f64,
                            sn * last_position.y as f64,
                            1.0,
                        ]) as f32,
                    ),
            )
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // draw ontop of other frames
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .w_h(SIZE as f32, SIZE as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.1));
    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
