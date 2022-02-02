use cpu_v1::fluid_object::FluidCube;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    field: FluidCube,
    visc: f32,
    diff: f32,
    draw_dens: bool,
    draw_vel: bool,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.visc, &mut model.diff, key);
    match key {
        Key::D => model.draw_dens = !model.draw_dens,
        Key::V => model.draw_vel = !model.draw_vel,
        _other_key => {}
    }
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

    let scale = 0.5;
    let rect = app.window_rect();
    let wh = scale * rect.wh();

    let field = FluidCube::new((wh.x.floor() as usize, wh.y.floor() as usize), 1.0, 4);
    Model {
        field,
        visc: 0.1,
        diff: 0.0001,
        draw_dens: true,
        draw_vel: true,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();
    let pos = rect.xy();

    for i in -1..1 {
        for j in -1..1 {
            model.field.add_density(
                vec2(pos.x + i as f32, pos.y + j as f32),
                random::<f32>(),
                rect,
            );
        }
    }

    let angle = random::<f32>();
    model.field.add_velocity(
        vec2(pos.x as f32, pos.y as f32),
        vec2(angle.cos(), angle.sin()),
        rect,
    );

    model.field.step(model.diff, model.visc);
    dbg!(model.diff);
    dbg!(model.visc);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    if model.draw_dens {
        model.field.draw_dens(&draw, app.window_rect());
    }
    if model.draw_vel {
        model.field.draw_vel(&draw, app.window_rect());
    }

    draw.to_frame(app, &frame).unwrap();
}
