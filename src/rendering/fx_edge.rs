use geometry::point::Point;

// -----------------------------------------------------
// Edge
// -----------------------------------------------------
pub struct FXEdge {
    // DDA info for x
    pub x: i64,
    x_step: i64,
    numerator: i64,
    denominator: i64,
    error_term: i64,

    // current y and vertical count
    pub y: i64,
    pub height: i64,
}

impl FXEdge {
    pub fn new() -> Self {
        Self {
            x: 0,
            x_step: 0,
            numerator: 0,
            denominator: 0,
            error_term: 0,
            y: 0,
            height: 0,
        }
    }

    // pub fn with_points(top: &Point, bot: &Point) -> Self {
    //     let mut e = FXEdge::new();
    //     e.set(top, bot);
    //     e
    // }

    pub fn set(&mut self, top: &Point, bot: &Point) {
        let y = f64::ceil(top.y as f64);
        let y_end = f64::ceil(bot.y as f64);
        let height = y_end - y;

        if height != 0.0 {
            let d_n = (bot.y - top.y) as f64;
            let d_m = (bot.x - top.x) as f64;

            // The original "16" multiplier is removed due to higher precision
            // and because we aren't using fixed-point numbers.
            let initial_numerator =
                d_m * y - d_m * (top.y as f64) + d_n * (top.x as f64) - 1.0 + d_n;
            let (x, error_term) = floor_div_mod(initial_numerator, d_n);
            let (x_step, numerator) = floor_div_mod(d_m, d_n);

            let denominator = d_n as i64;

            self.x = x as i64;
            self.x_step = x_step as i64;
            self.numerator = numerator as i64;
            self.denominator = denominator;
            self.error_term = error_term as i64;
            self.y = y as i64;
            self.height = height as i64;
        } else {
            self.x = 0;
            self.x_step = 0;
            self.numerator = 0;
            self.denominator = 0;
            self.error_term = 0;
            self.y = 0; //y as i64;
            self.height = 0; //height as i64;
        }
    }

    pub fn step(&mut self) {
        // println!("step: {}, {}", self.x_step, self.x);
        self.x += self.x_step;
        self.y += 1;
        self.height -= 1; // This is a counter

        self.error_term += self.numerator;
        if self.error_term >= self.denominator {
            self.x += 1;
            self.error_term -= self.denominator;
        }
    }
}

#[inline]
fn floor_div_mod(numerator: f64, denominator: f64) -> (f64, f64) {
    // assert!(denominator > 0); // we assume it's positive
    if denominator == 0.0 {
        return (0.0, 0.0);
    }

    if numerator >= 0.0 {
        // positive case, C is okay
        (numerator / denominator, numerator % denominator)
    } else {
        // Numerator is negative, do the right thing
        let mut floor = -((-numerator) / denominator);
        let mut modu = (-numerator) % denominator;

        if modu != 0.0 {
            // there is a remainder
            floor -= 1.0;
            modu = denominator - modu;
        }

        (floor, modu)
    }
}
