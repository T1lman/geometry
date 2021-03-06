use crate::round::round;
use std::f64::consts::PI;
#[derive(Debug)]
pub struct Sphere {
    volume: f64,
    surface: f64,
    radius: f64,
}

impl Sphere {
    pub fn construct(radius: f64) -> Self {
        Self {
            volume: radius.powf(3.0) * PI,
            surface: 4.0 * radius.powf(2.0) * PI,
            radius,
        }
    }
    pub fn get_radius_from_volume(&self) -> f64 {
        let vol = self.volume;
        round((vol / PI).powf(1.0 / 3.0))
    }
    pub fn get_radius_from_surface(&self) -> f64 {
        let surface = self.surface;
        round(((surface / PI) / 4.0).sqrt())
    }
}
