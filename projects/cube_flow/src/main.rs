use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    frame.clear(DIMGRAY);
    let draw = app.draw();
    let t = app.elapsed_frames() as f32 * 0.1;
    let rect = app.window_rect();
    draw.background().color(PLUM);

    let w = rect.w() * 0.09;
    let margin = rect.w() * 0.01;
    let tris = (0..10)
        .flat_map(|j| {
            (0..10).flat_map(move |i| {
                let centre = vec3(
                    rect.left() + (w + margin) * (i as f32 + 0.5),
                    0.0,
                    rect.left() + (w + margin) * (j as f32 + 0.5),
                );
                let size = vec3(
                    w,
                    20.0 + 50.0 * ((t + (i + j) as f32 * 0.1).sin() ),
                    w,
                );
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
    draw.to_frame(app, &frame).unwrap();
}
