#[derive(Debug)]
pub struct Sphere {
    volume: f64,
    surface: f64,
}

impl Sphere {
    pub fn construct(radius: f64) -> Self {
        Self {
            volume: radius.powf(3.0) * std::f64::consts::PI,
            surface: 4.0 * radius.powf(2.0) * std::f64::consts::PI,
        }
    }
    pub fn get_radius_from_volume(&self) -> f64 {
        let vol = self.volume;
        let radius = ((((vol / std::f64::consts::PI).powf(1.0 / 3.0)) * 100.0).round()) / 100.0;
        radius
    }
    pub fn get_radius_from_surface(&self) -> f64 {
        let surface = self.surface;
        let radius = (((((surface / std::f64::consts::PI) / 4.0).sqrt()) * 100.0).round()) / 100.0;
        radius
    }
}
