use cpu_v1::fluid_object::{DensColor, FluidCube};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    scale: f32,
    fluid: FluidCube,
    dens_opt: DensOpt,
    vel_opt: VelOpt,
    dt: f32,
    iter: usize,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.dens_opt.visc, &mut model.vel_opt.diff, key);
    match key {
        Key::D => model.dens_opt.draw_dens = !model.dens_opt.draw_dens,
        Key::V => model.vel_opt.draw_vel = !model.vel_opt.draw_vel,
        _other_key => {}
    }
}

fn scaled_fluid_cube(scale: f32, rect: Rect) -> (usize, usize) {
    let wh = scale * rect.wh();
    (wh.x.floor() as usize, wh.y.floor() as usize)
}

fn resized(app: &App, model: &mut Model, _vec: Vec2) {
    let fluid = FluidCube::new(scaled_fluid_cube(model.scale, app.window_rect()));
    model.fluid = fluid;
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .resized(resized)
        .build()
        .unwrap();

    let scale = 0.5;
    let rect = app.window_rect();

    let fluid = FluidCube::new(scaled_fluid_cube(scale, rect));
    let dens_opt = DensOpt {
        draw_dens: true,
        dens_color: DensColor::new(0.5, 0.5, 1.0),
        visc: 0.001,
    };
    let vel_opt = VelOpt {
        diff: 0.001,
        line_length: 5.0,
        color: hsla(1.0, 1.0, 1.0, 1.0),
        draw_vel: true,
    };
    Model {
        scale,
        fluid,
        dens_opt,
        vel_opt,
        dt: 1.0,
        iter: 4,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();
    let pos = rect.xy();

    for i in -1..1 {
        for j in -1..1 {
            model.fluid.add_density(
                vec2(pos.x + i as f32, pos.y + j as f32),
                random::<f32>(),
                rect,
            );
        }
    }

    let angle = random::<f32>();
    model.fluid.add_velocity(
        vec2(pos.x as f32, pos.y as f32),
        vec2(angle.cos(), angle.sin()),
        rect,
    );

    model.fluid.step(
        model.vel_opt.diff,
        model.dens_opt.visc,
        model.dt,
        model.iter,
    );
}

struct DensOpt {
    draw_dens: bool,
    dens_color: DensColor,
    visc: f32,
}

struct VelOpt {
    diff: f32,
    draw_vel: bool,
    line_length: f32,
    color: Hsla,
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    if model.dens_opt.draw_dens {
        model
            .fluid
            .draw_dens(&draw, app.window_rect(), model.dens_opt.dens_color);
    }

    if model.vel_opt.draw_vel {
        model.fluid.draw_vel(
            &draw,
            app.window_rect(),
            model.vel_opt.line_length,
            model.vel_opt.color,
        );
    }

    draw.to_frame(app, &frame).unwrap();
}
