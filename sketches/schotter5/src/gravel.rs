use nannou::{
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
struct StoneNoise {
    x: NoiseLoop,
    y: NoiseLoop,
    rot: NoiseLoop,
}

const LINE_WIDTH: f32 = 0.06;
#[derive(Debug)]
struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
    stone_noise: StoneNoise,
    cycles: f32,
    t: f32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        let stone_noise = StoneNoise {
            x: NoiseLoop::new(-0.5, 0.5),
            y: NoiseLoop::new(-0.5, 0.5),
            rot: NoiseLoop::new(0.0, TAU),
        };
        Stone {
            x,
            y,
            x_offset: 0.0,
            y_offset: 0.0,
            rotation: 0.0,
            stone_noise,
            cycles: 0.0,
            t: 0.0,
        }
    }

    fn init_noise(&mut self, noise: Perlin) {
        self.stone_noise.x.init(noise, self.cycles);
        self.stone_noise.y.init(noise, self.cycles);
        self.stone_noise.rot.init(noise, self.cycles);
    }

    fn update(&mut self, noise: Perlin, disp_factor: f32, rot_factor: f32) {
        self.x_offset = disp_factor * self.stone_noise.x.value(self.t, noise, self.cycles);
        self.y_offset = disp_factor * self.stone_noise.y.value(self.t, noise, self.cycles);
        self.rotation = rot_factor * self.stone_noise.rot.value(self.t, noise, self.cycles);
    }

    fn draw(&self, draw: &Draw) {
        let cdraw = draw.x_y(self.x, self.y);
        cdraw
            .rect()
            .no_fill()
            .stroke(STEELBLUE)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(self.x_offset, self.y_offset)
            .rotate(self.rotation);
    }
}

#[derive(Debug)]
pub struct Gravel {
    rows: u32,
    cols: u32,
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

fn position_factor(_x: f32, y: f32, rows: u32, _cols: u32) -> f32 {
    y / rows as f32
}

impl Gravel {
    pub fn new(rows: u32, cols: u32, loop_length: u32) -> Self {
        let stones = (0..rows)
            .map(|y| (0..cols).map(move |x| Stone::new(x as f32, y as f32)))
            .flatten()
            .collect();

        Gravel {
            rows,
            cols,
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
                let factor = position_factor(stone.x, stone.y, self.rows, self.cols);
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
        for stone in &self.stones {
            stone.draw(draw);
        }
    }
}
