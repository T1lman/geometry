use std::f64::consts::PI;

#[derive(Debug)]
pub struct Cylinder {
    volume: f64,
    surface: f64,
    radius: f64,
    height: f64,
}

impl Cylinder {
    pub fn construct(radius: f64, height: f64) -> Self {
        Self {
            volume: radius.powf(2.0) * PI * height,
            surface: (2.0 * radius * PI * height) + (2.0 * PI * radius.powf(2.0)),
            radius: radius,
            height: height,
        }
    }
}
