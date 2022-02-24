use nannou::prelude::*;

use voronator::delaunator::Point;
use voronator::CentroidDiagram;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Thing {
    pos: Vec2,
    color: Hsl,
}

struct Model {
    things: Vec<Thing>,
    field_up: f32,
    field_left: f32,
}

const SIZE: usize = 500;
const COUNT: usize = 100;

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

    Model {
        things: (0..COUNT).map(|_| Thing::new()).collect(),
        field_up: 120.0,
        field_left: 1.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for thing in model.things.iter_mut() {
        thing.wiggle();
    }
}

impl Thing {
    fn new() -> Self {
        Thing {
            pos: SIZE as f32 * vec2(random::<f32>() - 0.5, random::<f32>() - 0.5),
            color: hsl(random::<f32>(), 0.5, 0.6),
        }
    }

    fn wiggle(&mut self) {
        self.pos += vec2(random::<f32>() - 0.5, random::<f32>() - 0.5);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    let points: Vec<(f64, f64)> = model
        .things
        .iter()
        .map(|x| (x.pos.x as f64, x.pos.y as f64))
        .collect();
    let diagram = CentroidDiagram::<Point>::from_tuple(&points).unwrap();
    for cell in diagram.cells {
        draw.polygon()
            .points(cell.points().iter().map(|x| (x.x as f32, x.y as f32)))
            .hsl(random::<f32>(), 0.5, 0.6);
    }

    draw.to_frame(app, &frame).unwrap();
}
