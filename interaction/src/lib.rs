use chrono::{Datelike, Utc};
use nannou::prelude::*;
use std::path::PathBuf;

pub fn frame_path(app: &App) -> PathBuf {
    save_path(app)
        .join(format!("{:03}", app.elapsed_frames()))
        .with_extension("png")
}

pub fn save_path(app: &App) -> PathBuf {
    let now = Utc::now();
    app.assets_path()
        .expect("Expected project path")
        .join("images")
        .join(format!(
            "{}-{:02}-{:02}",
            now.year(),
            now.month(),
            now.day()
        ))
        .join(app.exe_name().unwrap())
}

pub fn key_pressed(app: &App, up_down: &mut f32, left_right: &mut f32, key: Key) {
    match key {
        Key::S => app.main_window().capture_frame(frame_path(app)),
        Key::Up => *up_down += 0.001,
        Key::Down => {
            if up_down > &mut 0.0 {
                *up_down -= 0.001;
            }
        }
        Key::Right => *left_right += 0.001,
        Key::Left => {
            if *left_right > 0.0 {
                *left_right -= 0.001;
            }
        }
        _other_key => {}
    }
}
