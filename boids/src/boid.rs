
use nannou::{color::{Hsv},  prelude::{Vec2, vec2, Rect}, rand::random, Draw};

use crate::flock::FlockSettings;

#[derive(Clone, Copy, Debug)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Boid {
    pub fn new(position: Vec2) -> Self {
        Boid {
            position,
            velocity: (2.0 + 2.0 * random::<f32>())
                * vec2(random::<f32>() - 0.5, random::<f32>() - 0.5),
            acceleration: Vec2::ZERO,
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

    fn normal(&self, force: Vec2, top_speed: f32, max_acc: f32) -> Vec2 {
        if force == vec2(0.0, 0.0) {
            return force;
        }
        let top = force.clamp_length_min(top_speed);
        let diff = top - self.velocity;

        diff.clamp_length_max(max_acc)
    }

    pub fn flock(&mut self, boids: &[Boid], settings: FlockSettings) {
        let neighbours = self.find_neighbours(boids, settings.radius);

        self.steer(
            settings.ratios.align,
            settings.top_speed,
            settings.max_acc,
            self.alignment(&neighbours),
        );
        self.steer(
            settings.ratios.cohesion,
            settings.top_speed,
            settings.max_acc,
            self.cohesion(&neighbours),
        );
        self.steer(
            settings.ratios.seperation,
            settings.top_speed,
            settings.max_acc,
            self.seperation(&neighbours),
        );
    }

    fn steer(&mut self, ratio: f32, top_speed: f32, max_acc: f32, force: Vec2) {
        let normal_force = self.normal(force, top_speed, max_acc);
        self.apply_force(ratio * normal_force);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update(&mut self, top_speed: f32) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(top_speed);
        self.acceleration = Vec2::ZERO;
    }

    pub fn draw(&self, draw: &Draw, color: Hsv) {
        draw.ellipse().radius(2.0).xy(self.position).color(color);
    }

    pub fn check_edges(&mut self, rect: &Rect) {
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
