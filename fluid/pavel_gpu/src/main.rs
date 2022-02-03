use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Bloom {
    enabled: bool,
    iterations: usize,
    resolution: usize,
    intensity: f32,
    threshold: f32,
    soft_knee: f32,
}

struct Sunrays {
    enabled: bool,
    resolution: usize,
    weight: usize,
}

struct Config {
    sim_resolution: usize,
    dye_resolution: usize,
    capture_resolution: usize,
    density_dissipation: f32,
    velocity_dissipation: f32,
    pressure: f32,
    pressure_iterations: usize,
    curl: usize,
    splat_radius: f32,
    splat_force: usize,
    shading: bool,
    colorful: bool,
    color_update_speed: usize,
    paused: bool,
    back_color: Srgb,
    transparent: bool,
    bloom: Bloom,
    sunrays: Sunrays,
}

fn update_ui(config: &mut Config, ctx: Frame) {
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut config.disp_adj, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut config.rot_adj, 0.0..=5.0).text("Rotation"));
            ui.add(egui::Slider::new(&mut config.motion, 0.0..=1.0).text("Motion"));
            
    ui.add(egui::Slider::new(&mut config.dye_resolution, 256..=1024).text("quality"));
    ui.add(egui::Slider::new(&mut config.sim_resolution, 32..=256 ).text("sim resolution");
    ui.add(egui::Slider::new(&mut config.density_dissipation, 0..=4.0).text("density diffusion");
    ui.add(egui::Slider::new(&mut config.velocity_dissipation, 0..=4.0).text("velocity diffusion");
    ui.add(egui::Slider::new(&mut config.pressure, 0.0..=1.0).text("pressure");
    ui.add(egui::Slider::new(&mut config.curl, 0..=50).text("vorticity");
    ui.add(egui::Slider::new(&mut config.splat_radius, 0.01..=1.0).text("splat radius");
    ui.add(egui::Checkbox::new(&mut config.shading).text("shading");
    ui.add(egui::Checkbox::new(&mut config.colorful).text("colorful");
    ui.add(egui::Checkbox::new(&mut config.paused).text("paused").listen();

    ui.add(egui::CollapsingHeader::new("Bloom")
    .show(ui, |ui| {
        ui.label("Bloom");
        ui.add(egui::Checkbox::new(&mut config.bloom.enabled).text("enabled");
        ui.add(egui::Slider::new(&mut config.bloom.intensity, 0.1..=2.0).text("intensity");
        ui.add(egui::Slider::new(&mut config.bloom.threshold, 0.0..=1.0).text("threshold");
    });)
    ui.add(egui::CollapsingHeader::new("Sunrays")
    .show(ui, |ui| {
        ui.label("Sunrays");
        ui.add(egui::Checkbox::new(&mut config.sunrays.enabled).text("enabled");
        ui.add(egui::Slider::new(&mut config.weight, 0.3..=1.0).text("weight");
    });)

    nannou_egui::edit_color(ui, &mut config.back_color);

        });
}


fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}
struct Model {
    config: Config,
    main_window: WindowId
    ui: Egui,
    data: Data
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(frame_path(app))
            }
        }
        _other_key => {}
    }
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
    let ui = Egui::from_window(&ui_window_ref);

    let config = Config {
        sim_resolution: 128,
        dye_resolution: 1024,
        capture_resolution: 512,
        density_dissipation: 1,
        velocity_dissipation: 0.2,
        pressure: 0.8,
        pressure_iterations: 20,
        curl: 30,
        splat_radius: 0.25,
        splat_force: 6000,
        shading: true,
        colorful: true,
        color_update_speed: 10,
        paused: false,
        back_color:BLACK,
        transparent: false,
        bloom: Some(Bloom{
        iterations: 8,
        resolution: 256,
        intensity: 0.8,
        threshold: 0.6,
        soft_knee: 0.7,})
        sunrays: Some(Sunrays{
            resolution: 196,
            weight: 1.0,
        })
    }

    Model {
        ui,
        main_window,
        config,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let Model {
        ref mut egui,
        ref mut config,
        ref mut data,
        ..
    } = *model;
    update_ui(config);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
