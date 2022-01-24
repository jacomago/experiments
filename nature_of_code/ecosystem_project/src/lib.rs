use nannou::prelude::*;

pub mod animal;

pub fn random_position(rect: Rect) -> Point2 {
    pt2(
        random_range(rect.left(), rect.right()),
        random_range(rect.bottom(), rect.top()),
    )
}
