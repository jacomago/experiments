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
        .flat_map(|i| {
            let centre = vec3(rect.left() + (w + margin) * (i as f32 + 0.5), 0.0, 0.0);
            let size = vec3(w, 20.0 + 50.0 * (t + i as f32 * 0.1).sin(), 20.0);
            geom::Cuboid::from_xyz_whd(centre, size)
                .triangles_iter().collect::<Vec<_>>()
        })
        .map(|tri| tri.map_vertices(|v| (v, STEELBLUE)));
    draw.mesh()
        .tris_colored(tris)
        .z_radians(0.33)
        .x_radians(0.33);
    draw.to_frame(app, &frame).unwrap();
}
