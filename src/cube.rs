#[derive(Debug)]
pub struct Cube {
    volume: f64,
    surface: f64,
    base_length: f64,
}

impl Cube {
    pub fn construct(base_length: f64) -> Self {
        Self {
            volume: base_length.powf(3.0),
            surface: 6.0 * base_length.powf(2.0),
            base_length: base_length,
        }
    }
}
