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
    ball: Ball,
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

struct Ball {
    position: Point2,
    speed: Vec2,
}

impl Ball {
    fn update(&mut self) {
        self.position += self.speed;
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.position).color(STEELBLUE);
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
        ball: Ball {
            position: pt2(0.0, 0.0),
            speed: vec2(1.0, 3.3),
        },
        fields: Fields { field: 120.0 },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.ball.update();
    model.ball.check_edges(&app.window_rect());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    model.ball.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
