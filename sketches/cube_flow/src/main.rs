use nannou::prelude::*;
use std::path::PathBuf;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    main_window: WindowId,
    frames_dir: PathBuf,
    cur_frame: u32,
    recording: bool,
    period_length: u32,
}

const SIZE: u32 = 1200;
fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE, SIZE)
        .view(view)
        .build()
        .unwrap();

    let frames_dir = app
        .assets_path()
        .expect("Expected project path")
        .join("images")
        .join("gif")
        .join("output")
        .join(app.exe_name().unwrap());
    let recording = false;
    let cur_frame = 0;

    let period_length = 50;
    Model {
        main_window,
        frames_dir,
        cur_frame,
        recording,
        period_length,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.recording {
        model.cur_frame += 1;
        if model.cur_frame == 3 * model.period_length + 1 {
            model.recording = false;
            model.frames_dir = model.frames_dir.join(format!("{}", random_ascii()));
            model.cur_frame = 0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
    let draw = app.draw();
    let t = app.elapsed_frames() as f32 * 0.02;
    let rect = app.window_rect();
    draw.background().color(PLUM);

    let w = rect.w() * 0.09;
    let margin = rect.w() * 0.01;
    let x_pos = rect.left() + (w + margin) * 10.5;
    let max_d = vec3(x_pos, 0.0, x_pos).length_squared();
    let tris = (0..10)
        .flat_map(|j| {
            (0..10).flat_map(move |i| {
                let centre = vec3(
                    rect.left() + (w + margin) * (i as f32 + 0.5),
                    0.0,
                    rect.left() + (w + margin) * (j as f32 + 0.5),
                );
                let d = map_range(centre.length_squared(), 0.0, max_d, -PI, PI);
                let height = 200.0 + 100.0 * ((-TAU * t + d).sin());
                let size = vec3(w, height, w);
                geom::Cuboid::from_xyz_whd(centre, size)
                    .faces_iter()
                    .enumerate()
                    .map(|f| {
                        (
                            f.1.triangles_iter(),
                            hsl(0.8, 0.8, map_range(f.0, 0, 8, 0.2, 0.4)),
                        )
                    })
                    .collect::<Vec<_>>()
            })
        })
        .flat_map(|tri_color| {
            tri_color
                .0
                .map(move |t| t.map_vertices(|v| (v, tri_color.1)))
        });
    draw.scale(0.5)
        .mesh()
        .tris_colored(tris)
        .y_radians(TAU * 0.125)
        .x_radians(TAU * 0.125);

    if model.recording {
        let filename = model
            .frames_dir
            .join(format!("cube_flow{:>04}", model.cur_frame))
            .with_extension("png");

        if let Some(window) = app.window(model.main_window) {
            window.capture_frame(filename);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
