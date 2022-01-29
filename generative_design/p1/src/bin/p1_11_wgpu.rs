use nannou::prelude::*;

use nannou::image;

fn main() {
    nannou::app(model).run();
}

struct Model {
    field_up: f32,
    field_left: f32,
    texture: wgpu::Texture,
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

    let window = app.main_window();
    let wrect = app.window_rect();
    let texture = wgpu::TextureBuilder::new()
        .size([wrect.w() as u32, wrect.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());
    Model {
        field_up: 120.0,
        field_left: 1.0,
        texture,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let mouse = app.mouse.position();
    let wrect = app.window_rect();

    let step = vec2(
        (mouse.x - wrect.left()).max(10.0),
        (wrect.top() - mouse.y).max(10.0),
    );

    let image = image::ImageBuffer::from_fn(wrect.w() as u32, wrect.h() as u32, |x, y| {
        let h = map_range(
            step.x * (x as f32 / step.x).floor(),
            0.0,
            wrect.w(),
            0.0,
            1.0,
        );
        let s = map_range(
            wrect.h() - step.y * (y as f32 / step.x).floor(),
            0.0,
            wrect.h(),
            0.0,
            1.0,
        );

        let c: Srgba = hsla(h, s, 0.5, 1.0).into();
        nannou::image::Rgba([
            map_range(c.red, 0.0, 1.0, 0, std::u8::MAX),
            map_range(c.green, 0.0, 1.0, 0, std::u8::MAX),
            map_range(c.blue, 0.0, 1.0, 0, std::u8::MAX),
            std::u8::MAX,
        ])
    });

    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        flat_samples.as_slice(),
    );

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
