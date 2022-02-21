use crate::theorem_of_pythagoras::RightTriangle;

#[derive(Debug)]
pub struct Pyramid {
    volume: f64,
    surface: f64,
    base_length: f64,
    height: f64,
}

impl Pyramid {
    pub fn construct(base_length: f64, height: f64) -> Self {
        let hypo =
            RightTriangle::construct_without_hypothenuse(height, base_length / 2.0).hypothenuse;
        Self {
            volume: base_length.powf(2.0) * height * 1.0 / 3.0,
            surface: base_length.powf(2.0) + 2.0 * base_length * hypo,
            base_length,
            height,
        }
    }
}
