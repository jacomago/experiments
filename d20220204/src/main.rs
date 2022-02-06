use nannou::{
    geom::Polygon,
    prelude::*,
    rand::{self, Rng},
};

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Thing {
    poly: Polygon<std::vec::IntoIter<Vec2>>,
    points: Vec<Point2>,
}

fn poly_points(number_of_sides: usize, radius: f32) -> Vec<Vec2> {
    (0..number_of_sides)
        .map(|i| {
            let fract = i as f32 / number_of_sides as f32;
            let radian = TAU * fract;
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            pt2(x, y)
        })
        .collect()
}

fn gen_line_points(step: f32, poly: &[Vec2]) -> Vec<Vec2> {
    let mut points = Vec::new();

    let mut ratio = step;
    while ratio < 1.0 {
        let mut previous_corner = None;
        let mut first_corner = Vec2::ZERO;
        for corner in poly {
            if previous_corner == None {
                previous_corner = Some(corner);
                first_corner = *corner;
                continue;
            }

            let direction = *corner - *previous_corner.unwrap();

            points.push(*previous_corner.unwrap() + direction * ratio);
            previous_corner = Some(corner);
        }
        points.push(first_corner);
        ratio += step;
    }
    points
}

impl Thing {
    fn new(radius: f32, number_of_sides: usize, step: f32) -> Self {
        let poly = poly_points(number_of_sides, radius);
        let points = gen_line_points(step, &poly);
        Thing {
            poly: Polygon::new(poly),
            points,
        }
    }

    fn draw(&self, draw: &Draw, angle: f32) {
        draw.polyline()
            .points(self.poly.points.clone())
            .join_round()
            .rotate(angle)
            .color(WHITE);
        draw.polyline()
            .points(self.points.clone())
            .rotate(angle)
            .color(WHITE);
    }
}

struct Model {
    thing: Thing,
    number_of_sides: usize,
    radius: f32,
    step: f32,
    angle: f32,
    side_change: usize,
    max_sides: usize,
    field_up: f32,
}

impl Model {
    fn update(&mut self) {
        self.thing = Thing::new(self.radius, self.number_of_sides, self.step);
    }
}
const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.step, key);
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

    let number_of_sides = 4;
    let step = 0.1;
    let radius = SIZE as f32 / 2.5;
    let angle = 0.0;

    Model {
        thing: Thing::new(radius, number_of_sides, step),
        radius,
        number_of_sides,
        step,
        angle,
        side_change: 20,
        max_sides: 10,
        field_up: 0.1,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.angle += model.field_up;
    if app.elapsed_frames() % model.side_change as u64 == 0 {
        model.number_of_sides = rand::thread_rng().gen_range(4..model.max_sides);
        model.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    model.thing.draw(&draw, model.angle);

    draw.to_frame(app, &frame).unwrap();
}
