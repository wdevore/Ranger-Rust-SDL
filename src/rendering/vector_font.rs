use geometry::point::Point;

pub struct VectorLine(f64, f64, f64, f64);

pub struct VectorGlyph {
    lines: Vec<Point>,
}

impl VectorGlyph {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn add_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.lines.push(Point::from_xy(x1, y1));
        self.lines.push(Point::from_xy(x2, y2));
    }

    pub fn get_lines(&self) -> &Vec<Point> {
        &self.lines
    }
}

// VectorFont is a collection of glyphs where each glyph is a collection
// of lines.
pub struct VectorFont {
    glyphs: Vec<VectorGlyph>,

    horizontal_offset: f64,
    vertical_offset: f64,
    scale: f64,
}

impl VectorFont {
    pub fn new() -> Self {
        let mut vf = Self {
            glyphs: Vec::new(),
            horizontal_offset: 1.2,
            vertical_offset: 1.2,
            scale: 3.0,
        };

        VectorFont::construct(&mut vf);

        vf
    }

    pub fn get_horz_offset(&self) -> f64 {
        self.horizontal_offset
    }

    pub fn get_vert_offset(&self) -> f64 {
        self.vertical_offset
    }

    pub fn get_scale(&self) -> f64 {
        self.scale
    }

    pub fn get_glyph(&self, c: char) -> &VectorGlyph {
        match c {
            'A' => &self.glyphs[0],
            'B' => &self.glyphs[1],
            'C' => &self.glyphs[2],
            'D' => &self.glyphs[3],
            'E' => &self.glyphs[4],
            'F' => &self.glyphs[5],
            'G' => &self.glyphs[6],
            'H' => &self.glyphs[7],
            'I' => &self.glyphs[8],
            'J' => &self.glyphs[9],
            'K' => &self.glyphs[10],
            'L' => &self.glyphs[11],
            'M' => &self.glyphs[12],
            'N' => &self.glyphs[13],
            'O' => &self.glyphs[14],
            'P' => &self.glyphs[15],
            'Q' => &self.glyphs[16],
            'R' => &self.glyphs[17],
            'S' => &self.glyphs[18],
            'T' => &self.glyphs[19],
            'U' => &self.glyphs[20],
            'V' => &self.glyphs[21],
            'W' => &self.glyphs[22],
            'X' => &self.glyphs[23],
            'Y' => &self.glyphs[24],
            'Z' => &self.glyphs[25],
            '0' => &self.glyphs[26],
            '1' => &self.glyphs[27],
            '2' => &self.glyphs[28],
            '3' => &self.glyphs[29],
            '4' => &self.glyphs[30],
            '5' => &self.glyphs[31],
            '6' => &self.glyphs[32],
            '7' => &self.glyphs[33],
            '8' => &self.glyphs[34],
            '9' => &self.glyphs[35],
            '=' => &self.glyphs[36],
            ',' => &self.glyphs[37],
            '.' => &self.glyphs[38],
            '/' => &self.glyphs[39],
            '!' => &self.glyphs[40],
            ':' => &self.glyphs[41],
            '_' => &self.glyphs[42],
            '-' => &self.glyphs[43],
            ' ' => &self.glyphs[44],
            _ => &self.glyphs[0],
        }
    }

    fn construct(vf: &mut VectorFont) {
        // A
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, 0.0, -1.0);
        glyph.add_line(0.5, 0.0, 0.0, -1.0);
        glyph.add_line(-0.3, -0.4, 0.3, -0.4);
        vf.glyphs.push(glyph);

        // B
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.25, 0.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.25, -1.0);
        glyph.add_line(0.25, -1.0, 0.5, -0.85);
        glyph.add_line(0.5, -0.85, 0.5, -0.65);
        glyph.add_line(0.5, -0.65, 0.5, -0.55);
        glyph.add_line(0.50, -0.55, 0.25, -0.5);
        glyph.add_line(0.25, -0.50, 0.5, -0.45);
        glyph.add_line(0.5, -0.45, 0.5, -0.35);
        glyph.add_line(0.5, -0.35, 0.25, -0.0);
        glyph.add_line(-0.5, -0.5, 0.25, -0.5);
        vf.glyphs.push(glyph);

        // C
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.5, -0.25, 0.25, 0.0);
        glyph.add_line(0.25, 0.0, -0.45, 0.0);
        glyph.add_line(-0.45, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.25, -0.5, -0.75);
        glyph.add_line(-0.5, -0.75, -0.45, -1.0);
        glyph.add_line(-0.45, -1.0, 0.25, -1.0);
        glyph.add_line(0.25, -1.0, 0.5, -0.75);
        vf.glyphs.push(glyph);

        // D
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.5, -0.25, 0.25, 0.0);
        glyph.add_line(0.25, 0.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.25, -1.0);
        glyph.add_line(0.25, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.75, 0.5, -0.25);
        vf.glyphs.push(glyph);

        // E
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.5, 0.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(-0.5, -0.5, 0.40, -0.5);
        vf.glyphs.push(glyph);

        // F
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(-0.5, -0.5, 0.4, -0.5);
        vf.glyphs.push(glyph);

        // G
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.0, -0.5, 0.4, -0.5);
        glyph.add_line(0.4, -0.5, 0.5, -0.4);
        glyph.add_line(0.5, -0.4, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(0.4, 0.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.45, -1.0);
        glyph.add_line(0.45, -1.0, 0.5, -0.75);
        vf.glyphs.push(glyph);

        // H
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(0.5, 0.0, 0.5, -1.0);
        glyph.add_line(-0.5, -0.5, 0.5, -0.5);
        vf.glyphs.push(glyph);

        // I
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, 0.5, 0.0);
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(0.0, 0.0, 0.0, -1.0);
        vf.glyphs.push(glyph);

        // J
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.3, -0.75, -0.3, -1.0);
        glyph.add_line(-0.3, -1.0, 0.5, -1.0);
        glyph.add_line(0.5, -1.0, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        vf.glyphs.push(glyph);

        // K
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -0.5, 0.4, -1.0);
        glyph.add_line(-0.5, -0.5, 0.5, 0.0);
        vf.glyphs.push(glyph);

        // L
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, 0.0, 0.4, 0.0);
        vf.glyphs.push(glyph);

        // M
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.0, -0.5);
        glyph.add_line(0.0, -0.5, 0.5, -1.0);
        glyph.add_line(0.5, 0.0, 0.5, -1.0);
        vf.glyphs.push(glyph);

        // N
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.5, 0.0);
        glyph.add_line(0.5, 0.0, 0.5, -1.0);
        vf.glyphs.push(glyph);

        // O
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.25, -0.5, -0.75);
        glyph.add_line(-0.5, -0.75, -0.4, -1.0);
        glyph.add_line(-0.4, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.75, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        vf.glyphs.push(glyph);

        // P
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.85);
        glyph.add_line(0.5, -0.85, 0.5, -0.65);
        glyph.add_line(0.5, -0.65, 0.4, -0.5);
        glyph.add_line(0.4, -0.5, -0.5, -0.5);
        vf.glyphs.push(glyph);

        // Q
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.25, -0.5, -0.75);
        glyph.add_line(-0.5, -0.75, -0.4, -1.0);
        glyph.add_line(-0.4, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.75, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(0.0, -0.5, 0.7, 0.2);
        vf.glyphs.push(glyph);

        // R
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.85);
        glyph.add_line(0.5, -0.85, 0.5, -0.65);
        glyph.add_line(0.5, -0.65, 0.4, -0.5);
        glyph.add_line(0.4, -0.5, -0.5, -0.5);
        glyph.add_line(0.2, -0.5, 0.5, -0.0);
        vf.glyphs.push(glyph);

        // S
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.5, -0.5, -0.75);
        glyph.add_line(-0.5, -0.75, -0.4, -1.0);
        glyph.add_line(-0.4, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.5, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(-0.5, -0.5, 0.5, -0.5);
        vf.glyphs.push(glyph);

        // T
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(0.0, 0.0, 0.0, -1.0);
        vf.glyphs.push(glyph);

        // U
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.25, -0.5, -1.0);
        glyph.add_line(0.5, -1.0, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        vf.glyphs.push(glyph);

        // V
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.0, 0.0);
        glyph.add_line(0.0, 0.0, 0.5, -1.0);
        vf.glyphs.push(glyph);

        // W
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, 0.0, -0.5);
        glyph.add_line(0.0, -0.5, 0.5, 0.0);
        glyph.add_line(0.5, -1.0, 0.5, 0.0);
        vf.glyphs.push(glyph);

        // X
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.5, 0.0);
        glyph.add_line(-0.5, 0.0, 0.5, -1.0);
        vf.glyphs.push(glyph);

        // Y
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.0, -0.5);
        glyph.add_line(0.0, -0.5, 0.5, -1.0);
        glyph.add_line(0.0, -0.5, 0.0, 0.0);
        vf.glyphs.push(glyph);

        // Z
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(0.5, -1.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, 0.5, 0.0);
        vf.glyphs.push(glyph);

        // 0
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.25, -0.5, -0.75);
        glyph.add_line(-0.5, -0.75, -0.4, -1.0);
        glyph.add_line(-0.4, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.75, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(-0.45, -0.1, 0.45, -0.9);
        vf.glyphs.push(glyph);

        // 1
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.2, -0.8, 0.0, -1.0);
        glyph.add_line(0.0, -1.0, 0.0, 0.0);
        glyph.add_line(-0.5, 0.0, 0.5, 0.0);
        vf.glyphs.push(glyph);

        // 2
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.75, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, 0.5, 0.0);
        vf.glyphs.push(glyph);

        // 3
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(0.5, -1.0, 0.5, 0.0);
        glyph.add_line(0.5, 0.0, -0.5, 0.0);
        glyph.add_line(-0.4, -0.5, 0.5, -0.5);
        vf.glyphs.push(glyph);

        // 4
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.0, 0.0, 0.0, -1.0);
        glyph.add_line(0.0, -1.0, -0.5, -0.5);
        glyph.add_line(-0.5, -0.5, 0.5, -0.5);
        vf.glyphs.push(glyph);

        // 5
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.5, -1.0, 0.0, -1.0);
        glyph.add_line(0.0, -1.0, 0.0, -0.5);
        glyph.add_line(0.0, -0.5, 0.4, -0.5);
        glyph.add_line(0.4, -0.5, 0.5, -0.4);
        glyph.add_line(0.5, -0.4, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(0.4, 0.0, -0.5, 0.0);
        vf.glyphs.push(glyph);

        // 6
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.5, -1.0, -0.5, -0.5);
        glyph.add_line(-0.5, -0.5, 0.5, -0.5);
        glyph.add_line(0.5, -0.5, 0.5, 0.0);
        glyph.add_line(0.5, 0.0, -0.5, 0.0);
        glyph.add_line(-0.5, 0.0, -0.5, -0.5);
        vf.glyphs.push(glyph);

        // 7
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -1.0, 0.5, -1.0);
        glyph.add_line(0.5, -1.0, 0.0, 0.0);
        vf.glyphs.push(glyph);

        // 8
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.4, 0.0, -0.4, 0.0);
        glyph.add_line(-0.4, 0.0, -0.5, -0.25);
        glyph.add_line(-0.5, -0.25, -0.5, -0.75);
        glyph.add_line(-0.5, -0.75, -0.4, -1.0);
        glyph.add_line(-0.4, -1.0, 0.4, -1.0);
        glyph.add_line(0.4, -1.0, 0.5, -0.75);
        glyph.add_line(0.5, -0.75, 0.5, -0.25);
        glyph.add_line(0.5, -0.25, 0.4, 0.0);
        glyph.add_line(-0.5, -0.5, 0.5, -0.5);
        vf.glyphs.push(glyph);

        // 9
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, 0.0, 0.5, -0.5);
        glyph.add_line(-0.5, -0.5, 0.5, -0.5);
        glyph.add_line(0.5, -0.5, 0.5, -1.0);
        glyph.add_line(0.5, -1.0, -0.5, -1.0);
        glyph.add_line(-0.5, -1.0, -0.5, -0.5);
        vf.glyphs.push(glyph);

        // =
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.5, -0.3, 0.5, -0.3);
        glyph.add_line(-0.5, -0.7, 0.5, -0.7);
        vf.glyphs.push(glyph);

        // ,
        let mut glyph = VectorGlyph::new();
        glyph.add_line(0.0, -0.3, 0.0, -0.2);
        glyph.add_line(0.0, -0.2, -0.3, -0.0);
        vf.glyphs.push(glyph);

        // .
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.1, 0.0, -0.1, -0.1);
        glyph.add_line(-0.1, -0.1, 0.1, -0.1);
        glyph.add_line(0.1, -0.1, 0.1, 0.0);
        glyph.add_line(0.1, 0.0, -0.1, 0.0);
        vf.glyphs.push(glyph);

        // "/"
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.25, 0.0, 0.25, -1.0);
        vf.glyphs.push(glyph);

        // !
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.1, 0.0, -0.1, -0.1);
        glyph.add_line(-0.1, -0.1, 0.1, -0.1);
        glyph.add_line(0.1, -0.1, 0.1, 0.0);
        glyph.add_line(0.1, 0.0, -0.1, 0.0);
        glyph.add_line(0.0, -0.2, 0.0, -1.0);
        vf.glyphs.push(glyph);

        // :
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.1, 0.0, -0.1, -0.1);
        glyph.add_line(-0.1, -0.1, 0.1, -0.1);
        glyph.add_line(0.1, -0.1, 0.1, 0.0);
        glyph.add_line(0.1, 0.0, -0.1, 0.0);
        glyph.add_line(-0.1, -0.9, -0.1, -1.0);
        glyph.add_line(-0.1, -1.0, 0.1, -1.0);
        glyph.add_line(0.1, -1.0, 0.1, -0.9);
        glyph.add_line(0.1, -0.9, -0.1, -0.9);
        vf.glyphs.push(glyph);

        // _
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.45, -0.1, 0.45, -0.1);
        vf.glyphs.push(glyph);

        // -
        let mut glyph = VectorGlyph::new();
        glyph.add_line(-0.40, -0.5, 0.40, -0.5);
        vf.glyphs.push(glyph);

        // " " <-- space
        let glyph = VectorGlyph::new();
        vf.glyphs.push(glyph);
    }
}
