use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    field_up: f32,
    field_left: f32,
    tile_size: f32,
    step_size: f32,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);
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

    Model {
        field_up: 120.0,
        field_left: 1.0,
        tile_size: 10.0,
        step_size: 15.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    let tile_points = vec![
        vec2(0.75, 0.5),
        vec2(0.5, -0.5),
        vec2(-0.75, -0.5),
        vec2(-0.5, 0.5),
    ];

    let start = app.window_rect().top_left();
    let directions = vec![vec2(1.0, 0.0), vec2(0.0, -1.0)];
    let n = 100;
    (0..n).for_each(|i| {
        (0..n).for_each(|j| {
            let o = tile_points
                .iter()
                .map(|y| {
                    start
                        + *y * model.tile_size
                        + model.step_size * (i as f32 * directions[0] + j as f32 * directions[1])
                })
                .collect::<Vec<Vec2>>();

            draw.polyline().points(o).color(BLACK);
        })
    });

    draw.to_frame(app, &frame).unwrap();
}
