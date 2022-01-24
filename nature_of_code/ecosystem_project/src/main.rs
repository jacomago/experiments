use std::collections::{HashMap, HashSet};

use ecosystem_project::{
    animal::{closet_prey_position, Animal, Species, SpeciesName, Visual},
    random_position,
};
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Fields {
    field: f64,
}

struct Model {
    species: HashMap<SpeciesName, Species>,
    animals: Vec<Animal>,
    fields: Fields,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(format!(
            "{}/images/{app_name}.png",
            &app.exe_name().unwrap(),
            app_name = &app.exe_name().unwrap()
        )),
        Key::Up => model.fields.field += 0.001,
        Key::Down => {
            if model.fields.field > 0.0 {
                model.fields.field -= 0.001;
            }
        }
        Key::Right => model.fields.field += 1.0,
        Key::Left => {
            if model.fields.field > 0.0 {
                model.fields.field -= 0.1;
            }
        }
        _other_key => {}
    }
}

fn create_species() -> HashMap<SpeciesName, Species> {
    let mut species = HashMap::new();
    let name = SpeciesName::Mosquito;
    species.insert(
        name,
        Species::new(
            name,
            10.0,
            0.5,
            None,
            Visual {
                radius: 1.0,
                color: BLACK,
            },
        ),
    );

    let name = SpeciesName::Frog;
    species.insert(
        name,
        Species::new(
            name,
            5.0,
            0.5,
            Some(HashSet::from([
                SpeciesName::Mosquito,
                SpeciesName::WaterBoatman,
                SpeciesName::PondSkater,
            ])),
            Visual {
                radius: 7.0,
                color: GREEN,
            },
        ),
    );

    let name = SpeciesName::WaterBoatman;
    species.insert(
        name,
        Species::new(
            name,
            2.0,
            0.5,
            None,
            Visual {
                radius: 3.0,
                color: BLUE,
            },
        ),
    );

    let name = SpeciesName::Goldfish;
    species.insert(
        name,
        Species::new(
            name,
            1.0,
            0.01,
            Some(HashSet::from([
                SpeciesName::Mosquito,
                SpeciesName::WaterBoatman,
                SpeciesName::PondSkater,
            ])),
            Visual {
                radius: 10.0,
                color: ORANGE,
            },
        ),
    );

    let name = SpeciesName::Newt;
    species.insert(
        name,
        Species::new(
            name,
            0.1,
            0.05,
            Some(HashSet::from([
                SpeciesName::Mosquito,
                SpeciesName::WaterBoatman,
                SpeciesName::PondSkater,
            ])),
            Visual {
                radius: 5.0,
                color: BLACK,
            },
        ),
    );

    let name = SpeciesName::PondSkater;
    species.insert(
        name,
        Species::new(
            name,
            3.0,
            0.5,
            None,
            Visual {
                radius: 1.0,
                color: GREY,
            },
        ),
    );

    let name = SpeciesName::Mouse;
    species.insert(
        name,
        Species::new(
            SpeciesName::Mouse,
            0.0,
            0.0,
            None,
            Visual {
                radius: 0.0,
                color: BLACK,
            },
        ),
    );
    species
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

    let species = create_species();
    let mut animals = Vec::new();

    for spec in species.values() {
        for _ in 0..20 {
            animals.push(Animal::new(spec.name, random_position(app.window_rect())));
        }
    }

    Model {
        animals,
        species,
        fields: Fields { field: 120.0 },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut species_positions: Vec<(SpeciesName, Vec2)> = vec![];
    for a in &model.animals {
        species_positions.push((a.species, a.position()))
    }
    species_positions.push((SpeciesName::Mouse, app.mouse.position()));

    for animal in model.animals.iter_mut() {
        let species = &model.species[&animal.species];

        let prey_pos = closet_prey_position(animal.position(), &species_positions, &species.prey);
        animal.update(species, prey_pos);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    for animal in &model.animals {
        animal.draw(&draw, model.species[&animal.species].visual);
    }
    draw.to_frame(app, &frame).unwrap();
}
