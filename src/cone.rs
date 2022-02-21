use crate::theorem_of_pythagoras::RightTriangle;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Cone {
    volume: f64,
    surface: f64,
}

impl Cone {
    pub fn construct(radius: f64, height: f64) -> Self {
        let hypo = RightTriangle::construct_without_hypothenuse(height, radius).hypothenuse;
        Self {
            volume: 1.0 / 3.0 * PI * radius.powf(2.0) * height,
            surface: PI * radius.powf(2.0) + PI * radius * hypo,
        }
    }
}
