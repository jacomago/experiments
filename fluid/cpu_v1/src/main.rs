use cpu_v1::fluid_object::{DensColor, FluidCube};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    scale: f32,
    dens_opt: DensOpt,
    vel_opt: VelOpt,
    dt: f32,
    iter: usize,
}
struct Model {
    fluid: FluidCube,
    egui: Egui,
    settings: Settings,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(
        app,
        &mut model.settings.dens_opt.visc,
        &mut model.settings.vel_opt.diff,
        key,
    );
    match key {
        Key::D => model.settings.dens_opt.draw_dens = !model.settings.dens_opt.draw_dens,
        Key::V => model.settings.vel_opt.draw_vel = !model.settings.vel_opt.draw_vel,
        _other_key => {}
    }
}

fn scaled_fluid_cube(scale: f32, rect: Rect) -> (usize, usize) {
    let wh = scale * rect.wh();
    (wh.x.floor() as usize, wh.y.floor() as usize)
}

fn regen(scale: f32, rect: Rect) -> FluidCube {
    FluidCube::new(scaled_fluid_cube(scale, rect))
}

fn resized(app: &App, model: &mut Model, _vec: Vec2) {
    model.fluid = regen(model.settings.scale, app.window_rect());
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .resized(resized)
        .raw_event(raw_window_event)
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
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        scale,
        vel_opt,
        dens_opt,
        dt: 1.0,
        iter: 4,
    };
    Model {
        fluid,
        egui,
        settings,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut fluid,
        ..
    } = *model;

    let rect = app.window_rect();

    egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();
    egui::Window::new("Workshop window").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.dens_opt.visc, 0.0..=1.0).text("dens visc"))
            .changed();
        ui.add(egui::Slider::new(&mut settings.vel_opt.diff, 0.0..=1.0).text("vel diff"))
            .changed();
        ui.add(
            egui::Slider::new(&mut settings.vel_opt.line_length, 1.0..=20.0)
                .text("vel line length"),
        )
        .changed();
        ui.add(egui::Slider::new(&mut settings.dt, 0.0..=5.0).text("time step"))
            .changed();
        ui.add(egui::Slider::new(&mut settings.iter, 1..=20).text("iter"))
            .changed();

        let mut scale_changed = false;
        scale_changed |= ui
            .add(egui::Slider::new(&mut settings.scale, 0.1..=1.0).text("scale"))
            .changed();
        scale_changed |= ui.button("Generate").clicked();
        if scale_changed {
            *fluid = regen(settings.scale, rect);
        }
    });

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
        model.settings.vel_opt.diff,
        model.settings.dens_opt.visc,
        model.settings.dt,
        model.settings.iter,
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

    if model.settings.dens_opt.draw_dens {
        model
            .fluid
            .draw_dens(&draw, app.window_rect(), model.settings.dens_opt.dens_color);
    }

    if model.settings.vel_opt.draw_vel {
        model.fluid.draw_vel(
            &draw,
            app.window_rect(),
            model.settings.vel_opt.line_length,
            model.settings.vel_opt.color,
        );
    }

    draw.to_frame(app, &frame).unwrap();

    let _draw_to_frame = model.egui.draw_to_frame(&frame);
}
