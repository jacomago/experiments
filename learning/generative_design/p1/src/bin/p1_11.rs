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
}

const SIZE: usize = 200;

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
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let mouse = app.mouse.position();
    let wrect = app.window_rect();

    let step = vec2(
        (mouse.x - wrect.left()).max(10.0),
        (wrect.top() - mouse.y).max(10.0),
    );

    let rect = Rect::from_wh(step).align_left_of(wrect).align_top_of(wrect);

    draw.background().color(WHITE);

    let mut inc_x = 0.0;
    while inc_x < wrect.w() {
        let mut inc_y = 0.0;
        while inc_y < wrect.h() {
            let rect = rect.shift_x(inc_x).shift_y(-inc_y);
            let h = map_range(inc_x, 0.0, wrect.w(), 0.0, 1.0);
            let s = map_range(wrect.h() - inc_y, 0.0, wrect.h(), 0.0, 1.0);
            draw.rect().xy(rect.xy()).wh(rect.wh()).hsl(h, s, 0.5);
            inc_y += step.y;
        }
        inc_x += step.x;
    }

    draw.to_frame(app, &frame).unwrap();
}
