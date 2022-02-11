use nannou::{
    color::Gradient,
    image::{self, RgbaImage},
    prelude::*,
};

pub struct LinearRenderer {
    hits: Vec<Vec<usize>>,
    w: usize,
    h: usize,
    img: RgbaImage,
}

impl LinearRenderer {
    pub fn new(w: usize, h: usize) -> Self {
        let hits = vec![vec![0; w]; h];
        let img = RgbaImage::new(w as u32, h as u32);
        LinearRenderer { hits, w, h, img }
    }

    pub fn add(&mut self, xy: Vec2) {
        let xint = xy.x.floor() as i32;
        let yint = xy.y.floor() as i32;
        if xint >= 0 && xint < self.w as i32 && yint >= 0 && yint < self.h as i32 {
            self.hits[yint as usize][xint as usize] += 1;
        }
    }

    pub fn render(&mut self, back: Srgba, fore: Srgba, gamma: f32) {
        let mx = *self
            .hits
            .iter()
            .map(|v| v.iter().max())
            .max()
            .unwrap()
            .unwrap() as f32;
        let mx = (mx + 1.0).ln();
        for (x, row) in self.hits.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                let pixel = if *col > 0 {
                    let alpha = ((*col as f32 + 1.0).ln() / mx).pow(gamma);
                    let grad = Gradient::new(vec![back.into_linear(), fore.into_linear()]);
                    let new_c = Srgba::from_linear(grad.get(alpha));
                    image::Rgba([
                        map_range(new_c.red, 0.0, 1.0, 0, std::u8::MAX),
                        map_range(new_c.green, 0.0, 1.0, 0, std::u8::MAX),
                        map_range(new_c.blue, 0.0, 1.0, 0, std::u8::MAX),
                        map_range(new_c.alpha, 0.0, 1.0, 0, std::u8::MAX),
                    ])
                } else {
                    image::Rgba([0, 0, 0, 0])
                };

                self.img.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
    pub fn img(&self) -> &RgbaImage {
        &self.img
    }
}
