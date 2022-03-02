use nannou::{
    geom::Tri,
    noise::{NoiseFn, Perlin},
    prelude::*,
};
use nannou_egui::egui::{self, Ui};

#[derive(Debug)]
struct NoiseLoop {
    min: f32,
    max: f32,
    seed: f64,
    start: f32,
}

impl NoiseLoop {
    fn new(min: f32, max: f32) -> Self {
        let seed = 1000.0 * random::<f64>();
        NoiseLoop {
            min,
            max,
            seed,
            start: 0.0,
        }
    }
    fn init(&mut self, noise: Perlin, diameter: f32) {
        self.start = 0.0;
        self.start = -self.value(0.0, noise, diameter);
    }

    fn value(&self, a: f32, noise: Perlin, diameter: f32) -> f32 {
        let x = map_range(a.cos(), -1.0, 1.0, 0.0, diameter as f64);
        let y = map_range(a.sin(), -1.0, 1.0, 0.0, diameter as f64);
        let r = noise.get([x, y, self.seed]);
        self.start - map_range(r, 0.0, 1.0, self.min, self.max)
    }
}

#[derive(Debug)]
struct NoiseLoopVec3 {
    x: NoiseLoop,
    y: NoiseLoop,
    z: NoiseLoop,
}

impl NoiseLoopVec3 {
    fn new(min: Vec3, max: Vec3) -> Self {
        NoiseLoopVec3 {
            x: NoiseLoop::new(min.x, max.x),
            y: NoiseLoop::new(min.y, max.y),
            z: NoiseLoop::new(min.z, max.z),
        }
    }

    fn init(&mut self, noise: Perlin, diameter: f32) {
        self.x.init(noise, diameter);
        self.y.init(noise, diameter);
        self.z.init(noise, diameter);
    }

    fn value(&self, a: f32, noise: Perlin, diameter: f32) -> Vec3 {
        vec3(
            self.x.value(a, noise, diameter),
            self.y.value(a, noise, diameter),
            self.z.value(a, noise, diameter),
        )
    }
}

#[derive(Debug)]
struct StoneNoise {
    xyz: NoiseLoopVec3,
    rot_theta: NoiseLoop,
    rot_phi: NoiseLoop,
}

#[derive(Debug)]
struct Stone {
    xyz: Vec3,
    xyz_offset: Vec3,
    rot_theta: f32,
    rot_phi: f32,
    stone_noise: StoneNoise,
    cycles: f32,
    t: f32,
    color: Hsla,
    size: f32,
}

impl Stone {
    fn new(xyz: Vec3, color: Hsla, size: f32) -> Self {
        let stone_noise = StoneNoise {
            xyz: NoiseLoopVec3::new(vec3(-0.5, -0.5, -0.5), vec3(0.5, 0.5, 0.5)),
            rot_theta: NoiseLoop::new(0.0, TAU),
            rot_phi: NoiseLoop::new(0.0, TAU),
        };
        Stone {
            xyz,
            xyz_offset: vec3(0.0, 0.0, 0.0),
            rot_theta: TAU * 0.125,
            rot_phi: TAU * 0.125,
            stone_noise,
            cycles: 0.0,
            t: 0.0,
            color,
            size,
        }
    }

    fn init_noise(&mut self, noise: Perlin) {
        self.stone_noise.xyz.init(noise, self.cycles);
        self.stone_noise.rot_theta.init(noise, self.cycles);
        self.stone_noise.rot_phi.init(noise, self.cycles);
    }

    fn update(&mut self, noise: Perlin, disp_factor: f32, rot_factor: f32) {
        self.xyz_offset = disp_factor * self.stone_noise.xyz.value(self.t, noise, self.cycles);
        self.rot_theta = rot_factor * self.stone_noise.rot_theta.value(self.t, noise, self.cycles);
        self.rot_phi = rot_factor * self.stone_noise.rot_phi.value(self.t, noise, self.cycles);
    }

    fn corners(&self) -> Vec<Vec3> {
        geom::Cuboid::from_xyz_whd(self.xyz, vec3(self.size, self.size, self.size))
            .shift(self.xyz_offset)
            .corners()
            .iter()
            .map(|x| vec3(x[0], x[1], x[2]))
            .collect()
    }
    fn tris_colored(&self) -> Vec<Tri<([f32; 3], Hsla)>> {
        geom::Cuboid::from_xyz_whd(self.xyz, vec3(self.size, self.size, self.size))
            .shift(self.xyz_offset)
            .faces_iter()
            .enumerate()
            .map(|f| {
                (
                    f.1.triangles_iter(),
                    hsla(
                        self.color.hue.to_radians() / TAU,
                        self.color.saturation,
                        map_range(f.0, 0, 8, self.color.lightness, 0.8),
                        self.color.alpha,
                    ),
                )
            })
            .flat_map(|t_c| t_c.0.map(move |t| t.map_vertices(|v| (v, t_c.1))))
            .collect()
    }

    fn draw(&self, draw: &Draw) {
        draw.mesh()
            .tris_colored(self.tris_colored())
            .y_radians(self.rot_theta)
            .x_radians(self.rot_phi);
        draw.polyline()
            .points(self.corners())
            .x_radians(self.rot_theta)
            .y_radians(self.rot_phi)
            .color(BLACK);
    }
}

#[derive(Debug)]
pub struct Gravel {
    rows: u32,
    cols: u32,
    depths: u32,
    disp_adj: f32,
    rot_adj: f32,
    motion: f32,
    noise: Perlin,
    stones: Vec<Stone>,
    min_loop: u32,
    loop_length: u32,
    smooth_factor: f32,
    t: u32,
}

fn position_factor(xyz: Vec3, rows: u32, cols: u32, depths: u32) -> f32 {
    (xyz.y / rows as f32) * (xyz.x / cols as f32) * (xyz.z / depths as f32)
}

impl Gravel {
    pub fn new(rows: u32, cols: u32, depths: u32, loop_length: u32) -> Self {
        let size = 1.0;
        let color = hsla(0.5, 0.5, 0.5, 0.8);
        let stones = (0..rows)
            .map(|y| {
                (0..cols).map(move |x| {
                    (0..depths)
                        .map(move |z| Stone::new(vec3(x as f32, y as f32, z as f32), color, size))
                })
            })
            .flatten()
            .flatten()
            .collect();

        Gravel {
            rows,
            cols,
            depths,
            stones,
            disp_adj: 1.5,
            rot_adj: 0.5,
            motion: 0.3,
            noise: Perlin::new(),
            min_loop: 50,
            loop_length,
            smooth_factor: 0.01,
            t: 0,
        }
    }

    pub fn update(&mut self) {
        let t_mod = self.t;
        for stone in &mut self.stones {
            if stone.cycles == 0.0 {
                if t_mod - self.min_loop > 0 && random_f32() > self.motion {
                    stone.cycles = self.smooth_factor * random_range(self.min_loop, t_mod) as f32;
                    stone.t = stone.cycles;
                    stone.init_noise(self.noise);

                    stone.update(self.noise, 0.0, 0.0);
                }
            } else {
                let factor = position_factor(stone.xyz, self.rows, self.cols, self.depths);
                let disp_factor = factor * self.disp_adj;
                let rot_factor = factor * self.rot_adj;
                stone.update(self.noise, disp_factor, rot_factor);
                stone.t -= self.smooth_factor;
            }
        }
        self.t += 1 % self.loop_length;
    }

    pub fn update_ui(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.disp_adj, 0.0..=5.0).text("Displacement"));
        ui.add(egui::Slider::new(&mut self.rot_adj, 0.0..=5.0).text("Rotation"));
        ui.add(egui::Slider::new(&mut self.motion, 0.0..=1.0).text("Motion"));
        let changed = ui
            .add(egui::Slider::new(&mut self.smooth_factor, 0.0..=0.1).text("Smooth Factor"))
            .changed();
        if changed {
            self.t = 0;
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.stones.iter().for_each(|s| s.draw(draw));
    }
}
