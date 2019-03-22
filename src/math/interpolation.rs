pub struct Interpolation {}

impl Interpolation {
    // Lerp returns a the value between min and max given t = 0->1
    #[inline(always)]
    pub fn lerp(min: f64, max: f64, t: f64) -> f64 {
        min * (1.0 - t) + max * t
    }

    // TODO new to review for negative ranges.
    // `linear` returns 0->1 for a "value" between min and max.
    // Generally used to map from view-space to unit-space
    #[inline(always)]
    pub fn linear(min: f64, max: f64, value: f64) -> f64 {
        if min < 0.0 {
            (value - max) / (min - max)
        } else {
            (value - min) / (max - min)
        }
    }
}
