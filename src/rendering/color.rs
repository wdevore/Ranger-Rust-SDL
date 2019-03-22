#[derive(Copy, Clone, Debug)]
pub struct Palette {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Palette {
    // ----------------------------------------------------
    // Constructors
    // ----------------------------------------------------
    #[inline]
    #[allow(non_snake_case)]
    pub fn RGB(r: u8, g: u8, b: u8) -> Palette {
        Palette { r, g, b, a: 0xff }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Palette {
        Palette { r, g, b, a }
    }

    /// Upper byte is ignored
    /// 0x__rrggbb
    #[inline]
    pub fn from_hex_rgb(color: u32) -> Palette {
        Palette {
            r: ((color & 0x00ff0000) >> 16) as u8,
            g: ((color & 0x0000ff00) >> 8) as u8,
            b: (color & 0x000000ff) as u8,
            a: 0xff,
        }
    }

    /// 0xrrggbbaa
    #[inline]
    pub fn from_hex_rgba(color: u32) -> Palette {
        Palette {
            r: ((color & 0xff000000) >> 24) as u8,
            g: ((color & 0x00ff0000) >> 16) as u8,
            b: ((color & 0x0000ff00) >> 8) as u8,
            a: (color & 0x000000ff) as u8,
        }
    }

    // ----------------------------------------------------
    // Color palette
    // ----------------------------------------------------
    #[inline]
    #[allow(non_snake_case)]
    pub fn DEFAULT() -> Palette {
        Palette::RGBA(0, 0, 0, 255)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn WHITE(alpha: u8) -> Palette {
        Palette::RGBA(255, 255, 255, alpha)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn BLACK() -> Palette {
        Palette::RGBA(0, 0, 0, 255)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn DARK_GRAY() -> Palette {
        Palette::RGBA(64, 64, 64, 255)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn GRAY() -> Palette {
        Palette::from_hex_rgb(0xAAAAAA)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn SILVER() -> Palette {
        Palette::from_hex_rgb(0xDDDDDD)
    }

    // ----------------------------------------------------
    // Yellow hues
    // ----------------------------------------------------
    #[inline]
    #[allow(non_snake_case)]
    pub fn ORANGE() -> Palette {
        Palette::RGB(255, 127, 0)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn SOFT_ORANGE() -> Palette {
        Palette::from_hex_rgb(0xFF851B)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn YELLOW() -> Palette {
        Palette::from_hex_rgb(0xFFDC00)
    }

    // ----------------------------------------------------
    // Red hues
    // ----------------------------------------------------
    #[inline]
    #[allow(non_snake_case)]
    pub fn RED() -> Palette {
        Palette::RGB(255, 0, 0)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn LIGHT_CLAY() -> Palette {
        Palette::from_hex_rgb(0xffaaaa)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn MAROON() -> Palette {
        Palette::from_hex_rgb(0x85144b)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn FUCHSIA() -> Palette {
        Palette::from_hex_rgb(0xF012BE)
    }

    // ----------------------------------------------------
    // Green hues
    // ----------------------------------------------------
    #[inline]
    #[allow(non_snake_case)]
    pub fn GREEN() -> Palette {
        Palette::RGB(0, 255, 0)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn SOFT_GREEN() -> Palette {
        Palette::from_hex_rgb(0x2ECC40)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn OLIVE() -> Palette {
        Palette::from_hex_rgb(0x3D9970)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn TEAL() -> Palette {
        Palette::from_hex_rgb(0x39CCCC)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn LIME() -> Palette {
        Palette::from_hex_rgb(0x01FF70)
    }

    // ----------------------------------------------------
    // Blue hues
    // ----------------------------------------------------
    #[inline]
    #[allow(non_snake_case)]
    pub fn BLUE() -> Palette {
        Palette::RGB(0, 0, 255)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn SOFT_BLUE() -> Palette {
        Palette::from_hex_rgb(0x0074D9)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn NAVY() -> Palette {
        Palette::from_hex_rgb(0x001f3f)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn AQUA() -> Palette {
        Palette::from_hex_rgb(0x7FDBFF)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn LIGHT_PURPLE() -> Palette {
        Palette::from_hex_rgb(0xaaaaff)
    }
}
