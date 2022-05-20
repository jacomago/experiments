use nannou::{
    image::{self, RgbaImage},
    prelude::*,
};

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    field_up: f32,
    field_left: f32,
    image_texture: wgpu::Texture,
    poster_img_text: wgpu::Texture,
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    interaction::key_pressed(app, &mut model.field_up, &mut model.field_left, key);
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("big_big1.png");

    let mut img = image::open(&img_path).unwrap().to_rgba8();
    let dim = img.dimensions();

    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(dim.0 * 2, dim.1)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let value = 3.0;
    let image_texture = wgpu::Texture::from_path(app, &img_path).unwrap();
    posterize_image(&mut img, value);
    let buf: image::DynamicImage = image::DynamicImage::ImageRgba8(img);
    let poster_img_text = wgpu::Texture::from_image(app, &buf);

    Model {
        image_texture,
        poster_img_text,
        field_up: 120.0,
        field_left: 1.0,
    }
}

fn posterize_image(img: &mut RgbaImage, factor: f32) {
    let dim = img.dimensions();
    let areas = std::u8::MAX as f32 / factor;
    let values = (std::u8::MAX as f32 - 1.0) / (factor - 1.0);
    for y in 1..(dim.1 - 1) {
        for x in 1..(dim.0 - 1) {
            let p = img.get_pixel(x, y);
            let new_p = posterize_pixel(p, areas, values);
            img.put_pixel(x, y, new_p);
        }
    }
}

fn poster_color(color: u8, areas: f32, values: f32) -> u8 {
    let area_f = color as f32 / areas;
    let mut area = area_f.round() as u8;
    if area as f32 > area_f {
        area -= 1;
    }
    let value_f = values * area as f32;
    let mut value = value_f.round() as u8;
    if value as f32 > value_f {
        value += 1;
    }
    value
}

fn posterize_pixel(p: &image::Rgba<u8>, areas: f32, values: f32) -> image::Rgba<u8> {
    image::Rgba([
        poster_color(p.0[0], areas, values),
        poster_color(p.0[1], areas, values),
        poster_color(p.0[2], areas, values),
        poster_color(p.0[3], areas, values),
    ])
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    let x = app.window_rect().left() + app.window_rect().w() / 4.0;
    draw = draw.x(x);
    draw.texture(&model.image_texture);
    draw = draw.x(app.window_rect().left() + app.window_rect().w());
    draw.texture(&model.poster_img_text);
    draw.to_frame(app, &frame).unwrap();
}
