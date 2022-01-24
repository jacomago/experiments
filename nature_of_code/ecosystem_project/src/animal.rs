use std::collections::HashSet;

use nannou::prelude::*;

#[derive(Clone, Copy)]
pub struct Visual {
    pub radius: f32,
    pub color: Srgb<u8>,
}

#[repr(usize)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum SpeciesName {
    Frog,
    Mosquito,
    PondSkater,
    Goldfish,
    WaterBoatman,
    Newt,
    Mouse,
}

pub struct Species {
    pub name: SpeciesName,
    top_speed: f32,
    acceleration_ratio: f32,
    pub prey: Option<HashSet<SpeciesName>>,
    pub visual: Visual,
}

impl Species {
    pub fn new(
        name: SpeciesName,
        top_speed: f32,
        acceleration_ratio: f32,
        prey: Option<HashSet<SpeciesName>>,
        visual: Visual,
    ) -> Self {
        Species {
            name,
            top_speed,
            acceleration_ratio,
            prey,
            visual,
        }
    }

    fn new_acceleration(&self, position: Position, closest_pos: Option<Point2>) -> Vec2 {
        match self.name {
            SpeciesName::Frog => {
                if position.velocity.length().abs() > 0.0
                    && (position.acceleration.angle() - position.velocity.angle()).abs()
                        < f32::EPSILON
                {
                    return -position.acceleration;
                }
                if let Some(closest) = closest_pos {
                    return (closest - position.position).normalize() * self.acceleration_ratio;
                }
                position.acceleration
            }
            SpeciesName::Mosquito => {
                if (position.velocity.length() - self.top_speed).abs() < f32::EPSILON {
                    return -random_acceleration(self.acceleration_ratio);
                }
                if position.velocity.length() == 0.0 {
                    return random_acceleration(self.acceleration_ratio);
                }
                position.acceleration + random_acceleration(self.acceleration_ratio)
            }
            SpeciesName::PondSkater => {
                if (position.velocity.length() - self.top_speed).abs() < f32::EPSILON {
                    return -self.acceleration_ratio * position.acceleration;
                }
                if position.velocity.length() == 0.0 {
                    return random_acceleration(self.acceleration_ratio);
                }
                position.acceleration
            }
            SpeciesName::Goldfish => {
                if let Some(closest) = closest_pos {
                    return (closest - position.position).normalize() * self.acceleration_ratio;
                }
                position.acceleration
            }
            SpeciesName::WaterBoatman => random_acceleration(self.acceleration_ratio),
            SpeciesName::Newt => random_acceleration(self.acceleration_ratio),
            SpeciesName::Mouse => self.acceleration_ratio * vec2(1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    acceleration: Vec2,
    velocity: Vec2,
    position: Point2,
}

impl Position {
    fn update(&mut self, new_acc: Vec2, top_speed: f32) {
        self.acceleration = new_acc;
        self.velocity += self.acceleration;

        self.velocity = self.velocity.clamp_length_max(top_speed);

        self.position += self.velocity;
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

pub fn closet_prey_position(
    current_pos: Point2,
    animals: &[(SpeciesName, Point2)],
    input_prey: &Option<HashSet<SpeciesName>>,
) -> Option<Point2> {
    if let Some(prey) = input_prey {
        let mut out = animals.first();
        let mut min = f32::INFINITY;
        for animal in animals.iter().filter(|x| prey.contains(&x.0)) {
            let new_min = (current_pos - animal.1).length_squared().abs();
            if new_min < min {
                min = new_min;
                out = Some(animal);
            }
        }
        return out.map(|x| x.1);
    }
    None
}
pub struct Animal {
    pub species: SpeciesName,
    position: Position,
}

fn random_acceleration(factor: f32) -> Point2 {
    vec2(
        factor * random_range(-1.0, 1.0),
        factor * random_range(-1.0, 1.0),
    )
}

impl Animal {
    pub fn new(species: SpeciesName, position: Point2) -> Self {
        Animal {
            species,
            position: Position {
                acceleration: vec2(0.0, 0.0),
                velocity: vec2(0.0, 0.0),
                position,
            },
        }
    }

    pub fn position(&self) -> Point2 {
        self.position.position
    }

    pub fn check_edges(&mut self, rect: &Rect) {
        self.position.check_edges(rect);
    }

    pub fn update(&mut self, species: &Species, close_pos: Option<Point2>) {
        self.position.update(
            species.new_acceleration(self.position, close_pos),
            species.top_speed,
        );
    }

    pub fn draw(&self, draw: &Draw, visual: Visual) {
        draw.ellipse()
            .xy(self.position.position)
            .radius(visual.radius)
            .color(visual.color);
    }
}
