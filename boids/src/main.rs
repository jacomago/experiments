use nannou::prelude::*;

use nannou_egui::{self, egui, Egui};

// TODO: Multiple boid types
// TODO: Better visuals
// TODO: SpatialSplit calc
// TODO: max acc and max speed in interface

// TODO: 3D???

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

#[derive(Clone, Copy, Debug)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
    max_acc: f32,
    color: Srgba,
}

impl Boid {
    fn new(position: Vec2, color: Srgba, top_speed: f32, max_acc: f32) -> Self {
        Boid {
            position,
            velocity: (2.0 + 2.0 * random::<f32>())
                * vec2(random::<f32>() - 0.5, random::<f32>() - 0.5),
            acceleration: Vec2::ZERO,
            top_speed,
            max_acc,
            color,
        }
    }

    fn find_neighbours(&self, boids: &[Boid], radius: f32) -> Vec<Boid> {
        boids
            .iter()
            .filter(|x| {
                self.position.distance(x.position) < radius
                    && !x.position.abs_diff_eq(self.position, 0.0001)
            })
            .copied()
            .collect()
    }

    fn seperation(&self, neighbours: &[Boid]) -> Vec2 {
        let mut avg = Vec2::ZERO;
        if neighbours.is_empty() {
            return avg;
        }

        for neighbour in neighbours {
            let d = self.position.distance(neighbour.position);
            let diff = self.position - neighbour.position;
            let diff = diff / d;
            let diff = if diff.is_nan() { Vec2::ZERO } else { diff };
            avg += diff;
        }
        avg / neighbours.len() as f32
    }

    fn alignment(&self, neighbours: &[Boid]) -> Vec2 {
        let mut avg = Vec2::ZERO;
        if neighbours.is_empty() {
            return avg;
        }
        for neighbour in neighbours {
            avg += neighbour.velocity;
        }
        avg / neighbours.len() as f32
    }

    fn cohesion(&self, neighbours: &[Boid]) -> Vec2 {
        let mut avg = Vec2::ZERO;
        if neighbours.is_empty() {
            return avg;
        }
        for neighbour in neighbours {
            avg += neighbour.position;
        }
        (avg / neighbours.len() as f32) - self.position
    }

    fn normal(&self, force: Vec2) -> Vec2 {
        if force == vec2(0.0, 0.0) {
            return force;
        }
        let top = force.clamp_length_min(self.top_speed);
        let diff = top - self.velocity;

        diff.clamp_length_max(self.max_acc)
    }

    fn flock(&mut self, boids: &[Boid], radius: f32, ratios: &Ratios) {
        let neighbours = self.find_neighbours(boids, radius);
        let align = self.alignment(&neighbours);
        let cohes = self.cohesion(&neighbours);
        let sep = self.seperation(&neighbours);

        let align = self.normal(align);
        let cohes = self.normal(cohes);
        let sep = self.normal(sep);

        self.steer(ratios.align * align);
        self.steer(ratios.cohesion * cohes);
        self.steer(ratios.seperation * sep);
    }

    fn steer(&mut self, desired: Vec2) {
        self.acceleration += desired;
    }

    fn update(&mut self) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.acceleration = Vec2::ZERO;
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .radius(2.0)
            .xy(self.position)
            .color(self.color);
    }

    fn check_edges(&mut self, rect: &Rect) {
        if self.position.x < rect.left() {
            self.position.x = rect.right();
        } else if self.position.x > rect.right() {
            self.position.x = rect.left()
        }

        if self.position.y < rect.bottom() {
            self.position.y = rect.top();
        } else if self.position.y > rect.top() {
            self.position.y = rect.bottom()
        }
    }
}

struct Ratios {
    align: f32,
    cohesion: f32,
    seperation: f32,
}
struct Settings {
    ratios: Ratios,
    radius: f32,
}
struct Model {
    main_window: WindowId,
    egui: Egui,
    boids: Vec<Boid>,
    settings: Settings,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(
        app,
        &mut model.settings.radius,
        &mut model.settings.ratios.align,
        key,
    );
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(280, 130)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window_ref = app.window(ui_window).unwrap();
    let egui = Egui::from_window(&ui_window_ref);

    let boids = (0..5)
        .map(|_x| {
            Boid::new(
                (SIZE / 2) as f32 * vec2(random::<f32>() - 0.5, random::<f32>() - 0.5),
                srgba(1.0, 1.0, 1.0, 1.0),
                10.0,
                0.2,
            )
        })
        .collect();
    let settings = Settings {
        ratios: Ratios {
            align: 0.0,
            cohesion: 0.0,
            seperation: 0.0,
        },
        radius: 40.0,
    };
    Model {
        main_window,
        egui,
        boids,
        settings,
    }
}
fn update_ui(egui: &mut Egui, settings: &mut Settings) {
    let ctx = egui.begin_frame();
    egui::Window::new("Workshop window").show(&ctx, |ui| {
        //vels
        ui.add(egui::Slider::new(&mut settings.ratios.align, 0.0..=1.0).text("align"))
            .changed();
        ui.add(egui::Slider::new(&mut settings.ratios.cohesion, 0.0..=1.0).text("cohesion"))
            .changed();
        ui.add(egui::Slider::new(&mut settings.ratios.seperation, 0.0..=1.0).text("sep"))
            .changed();
        ui.add(egui::Slider::new(&mut settings.radius, 2.0..=100.0).text("radius"))
            .changed();
    });
}
fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut boids,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    update_ui(egui, settings);

    let boids_copy = boids.clone();
    let rect = if let Some(window) = app.window(model.main_window) {
        window.rect()
    } else {
        app.window_rect()
    };
    for boid in boids.iter_mut() {
        boid.flock(&boids_copy, model.settings.radius, &model.settings.ratios);

        boid.update();
        boid.check_edges(&rect);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for boid in &model.boids {
        boid.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
