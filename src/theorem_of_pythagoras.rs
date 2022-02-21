#[derive(Debug)]
pub struct RightTriangle {
    pub hypothenuse: f64,
    pub opposite: f64,
    pub adjacent: f64,
}

impl RightTriangle {
    pub fn construct_with_hypothenuse(hypothenuse: f64, opposite: f64) -> Self {
        Self {
            hypothenuse: hypothenuse,
            opposite: opposite,
            adjacent: (hypothenuse.powf(2.0) - opposite.powf(2.0)).sqrt(),
        }
    }
    pub fn construct_without_hypothenuse(opposite: f64, adjacent: f64) -> Self {
        Self {
            hypothenuse: (opposite.powf(2.0) + adjacent.powf(2.0)).sqrt(),
            opposite: opposite,
            adjacent: adjacent,
        }
    }
}
