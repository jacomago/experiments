use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for i in 1..10 {
        let angle = i as f32 * 0.1 * TAU;
        draw.ellipse()
            .x_y(100.0 * angle.cos(), 100.0 * angle.sin())
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
