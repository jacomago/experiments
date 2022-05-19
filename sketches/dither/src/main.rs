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
    dith_img_text: wgpu::Texture,
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

    let image_texture = wgpu::Texture::from_path(app, &img_path).unwrap();
    dither_image(&mut img, 4.0);
    let buf: image::DynamicImage = image::DynamicImage::ImageRgba8(img);
    let dith_img_text = wgpu::Texture::from_image(app, &buf);
    Model {
        image_texture,
        dith_img_text,
        field_up: 120.0,
        field_left: 1.0,
    }
}

fn reduce_color(i: u8, factor: f32) -> u8 {
    ((std::u8::MAX as f32 / factor) * ((factor * i as f32) / (std::u8::MAX as f32)).floor()).floor()
        as u8
}

fn add_color_array(first: [u8; 4], second: [u8; 4], factor: f32) -> [u8; 4] {
    let mut new = [0; 4];
    for i in 0..4 {
        new[i] = (first[i] as f32 + second[i] as f32 * factor) as u8;
    }
    new
}

fn reduce_pixel(pixel: &image::Rgba<u8>, factor: f32) -> [u8; 4] {
    let mut new_color_array: [u8; 4] = [0; 4];
    for i in 0..4 {
        let p_color = pixel.0[i];
        new_color_array[i] = reduce_color(p_color, factor);
    }
    new_color_array
}

// Floyd-Steinberg dithering
fn dither_image(img: &mut RgbaImage, factor: f32) {
    let dim = img.dimensions();
    for y in 1..(dim.1 - 1) {
        for x in 1..(dim.0 - 1) {
            let p = img.get_pixel(x, y);
            let new_p = reduce_pixel(p, factor);
            let diff = add_color_array(p.0, new_p, -1.0);

            let n = image::Rgba(add_color_array(img.get_pixel(x + 1, y).0, diff, 7.0 / 16.0));
            img.put_pixel(x + 1, y, n);
            let n = image::Rgba(add_color_array(
                img.get_pixel(x - 1, y + 1).0,
                diff,
                3.0 / 16.0,
            ));
            img.put_pixel(x - 1, y + 1, n);
            let n = image::Rgba(add_color_array(img.get_pixel(x, y + 1).0, diff, 5.0 / 16.0));
            img.put_pixel(x, y + 1, n);
            let n = image::Rgba(add_color_array(
                img.get_pixel(x + 1, y + 1).0,
                diff,
                1.0 / 16.0,
            ));
            img.put_pixel(x + 1, y + 1, n);
        }
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    let x = app.window_rect().left() + app.window_rect().w() / 4.0;
    draw = draw.x(x);
    draw.texture(&model.image_texture);
    draw = draw.x(app.window_rect().left() + app.window_rect().w());
    draw.texture(&model.dith_img_text);
    draw.to_frame(app, &frame).unwrap();
}
