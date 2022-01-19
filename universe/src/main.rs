use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    position: Vec2,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        Thing { position: p }
    }
}

struct Model {
    things: Vec<Thing>,
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let mut things = Vec::new();
    let thing = Thing::new(vec2(0.0, 0.0));
    things.push(thing);

    Model { things, _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.elapsed_frames() as f32 / 60.0;

    draw.background().color(BLACK);

    for thing in model.things.iter() {
        draw.ellipse().xy(thing.position).radius(5.0).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
