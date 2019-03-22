use math::interpolation::Interpolation;

pub struct AngularMotion {
    // From and To values
    from: f64,
    to: f64,
    step_value: f64,

    // Caution:
    // Interpolation can sometimes be a tiny bit off at the wrap boundary
    // which renders as a small jitter at the boundary. This can
    // most noticable for orbit type nodes where the error is
    // magnified.
    // It is enabled by default.
    auto_wrap: bool,
}

impl AngularMotion {
    pub fn new() -> Self {
        Self {
            from: 0.0,
            to: 0.0,
            step_value: 0.0,
            auto_wrap: false,
        }
    }

    pub fn enable_auto_wrap(&mut self, wrap: bool) {
        self.auto_wrap = wrap;
    }

    pub fn set_step_value(&mut self, value: f64) {
        self.step_value = value;
    }

    pub fn set(&mut self, from: f64, to: f64) {
        self.from = from;
        self.to = to;
    }

    // step returns a the value between `from` and `to` given t = (0.0 -> 1.0)
    pub fn interpolate(&mut self, t: f64) -> f64 {
        let mut value = Interpolation::lerp(self.from, self.to, t);

        // Adjust value if wrapped
        if self.auto_wrap {
            if self.step_value > 0.0 {
                if value >= 360.0 {
                    // Wrap range back around
                    self.from = 0.0;
                    self.to = self.step_value;
                    // Calc new value from the adjusted range
                    value = Interpolation::lerp(self.from, self.to, t); //value - 360.0;
                }
            } else {
                if value <= 0.0 {
                    self.from = 359.0;
                    self.to = 359.0 + self.step_value;
                    value = Interpolation::lerp(self.from, self.to, t); //360.0 + value;
                }
            }
        }

        value
    }

    pub fn update(&mut self, _dt: f64) {
        self.from = self.to;
        self.to = self.to + self.step_value;
    }

    pub fn to_string(&self) -> String {
        format!("{} -> {}", self.from, self.to)
    }
}
