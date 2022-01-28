use std::path::PathBuf;

use nannou::{
    image::{self, DynamicImage, GenericImageView},
    prelude::*,
};

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}
enum SortMode {
    Hue,
    Saturation,
    Brightness,
    Grayscale,
    Null,
}
struct Model {
    img: DynamicImage,
    mode: SortMode,
    field_up: f32,
    field_left: f32,
}

const SIZE: usize = 300;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);

    match key {
        Key::Key1 => {
            model.mode = SortMode::Null;
        }
        Key::Key2 => {
            model.mode = SortMode::Hue;
        }
        Key::Key3 => {
            model.mode = SortMode::Saturation;
        }
        Key::Key4 => {
            model.mode = SortMode::Brightness;
        }
        Key::Key5 => {
            model.mode = SortMode::Grayscale;
        }
        _otherkey => (),
    }
}

fn get_image(path: PathBuf) -> DynamicImage {
    image::open(path).unwrap()
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

    let img = get_image(
        app.assets_path()
            .unwrap()
            .join("images")
            .join("test_colour.jpg"),
    );

    Model {
        img,
        mode: SortMode::Null,
        field_up: 120.0,
        field_left: 1.0,
    }
}

fn translate_type(input: image::Rgba<u8>) -> Srgba {
    srgba(
        input[0] as f32 / 255.0,
        input[1] as f32 / 255.0,
        input[2] as f32 / 255.0,
        input[3] as f32 / 255.0,
    )
}

fn get_colors(img: &DynamicImage, tile_count: usize, rect_size: f32) -> Vec<Srgba> {
    let mut colors = vec![];
    for y in 0..tile_count {
        for x in 0..tile_count {
            let p = vec2(
                x as f32 * rect_size + (rect_size * 0.5),
                y as f32 * rect_size + (rect_size * 0.5),
            );
            let c = translate_type(img.get_pixel(p.x as u32, p.y as u32));
            colors.push(c);
        }
    }
    colors
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn sort_colors(colors: &mut Vec<Srgba>, mode: &SortMode) {
    match mode {
        SortMode::Null => (),
        SortMode::Hue => colors.sort_by(|a, b| {
            let x: Hsl = (*a).into();
            let y: Hsl = (*b).into();
            x.hue.to_radians().partial_cmp(&y.hue.to_radians()).unwrap()
        }),
        SortMode::Saturation => colors.sort_by(|a, b| {
            let x: Hsl = (*a).into();
            let y: Hsl = (*b).into();
            x.saturation.partial_cmp(&y.saturation).unwrap()
        }),
        SortMode::Brightness => colors.sort_by(|a, b| {
            let x: Hsl = (*a).into();
            let y: Hsl = (*b).into();
            x.lightness.partial_cmp(&y.lightness).unwrap()
        }),
        SortMode::Grayscale => colors.sort_by(|a, b| a.alpha.partial_cmp(&b.alpha).unwrap()),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let mouse = app.mouse.position();
    let wrect = app.window_rect();

    let tile_count = (app.window_rect().w() / mouse.x.max(5.0)).floor() as usize;

    let step = vec2(wrect.w() / tile_count as f32, wrect.h() / tile_count as f32);

    let mut colors = get_colors(&model.img, tile_count, step.x);
    sort_colors(&mut colors, &model.mode);
    let rect = Rect::from_wh(step).align_left_of(wrect).align_top_of(wrect);

    draw.background().color(WHITE);

    let mut index = 0;
    let mut inc_x = 0.0;
    while inc_x < wrect.w() {
        let mut inc_y = 0.0;
        while inc_y < wrect.h() {
            let rect = rect.shift_x(inc_x).shift_y(-inc_y);
            draw.rect().xy(rect.xy()).wh(rect.wh()).color(colors[index]);
            inc_y += step.y;
            index += 1;
        }
        inc_x += step.x;
    }

    draw.to_frame(app, &frame).unwrap();
}
