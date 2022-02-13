use nannou::{color::Gradient, prelude::*};

pub mod renderer;

pub fn lerp_colors_duo(back: Srgba, fore: Srgba, alpha: f32) -> Srgba {
    let grad = Gradient::new(vec![back.into_linear(), fore.into_linear()]);
    Srgba::from_linear(grad.get(alpha))
}

pub fn lerp_colors(colors: &[Srgba], alpha: f32) -> Srgba {
    let grad = Gradient::new(colors.iter().map(|c| c.into_linear()));
    Srgba::from_linear(grad.get(alpha))
}

pub fn color_u8(c: Srgba) -> [u8; 4] {
    [
        map_range(c.red, 0.0, 1.0, 0, std::u8::MAX),
        map_range(c.green, 0.0, 1.0, 0, std::u8::MAX),
        map_range(c.blue, 0.0, 1.0, 0, std::u8::MAX),
        map_range(c.alpha, 0.0, 1.0, 0, std::u8::MAX),
    ]
}
