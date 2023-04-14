use crate::Vec3;

/// Macro to pack RGBA values into a u32
#[macro_export]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        (($r as u32 & 0xFF) << (8 * 0))
            | (($g as u32 & 0xFF) << (8 * 1))
            | (($b as u32 & 0xFF) << (8 * 2))
            | (($a as u32 & 0xFF) << (8 * 3))
    };
}

/// Macro to pack RGB values into a u32 with an alpha of 255
#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        (($r as u32 & 0xFF) << (8 * 0))
            | (($g as u32 & 0xFF) << (8 * 1))
            | (($b as u32 & 0xFF) << (8 * 2))
            | ((0xFF as u32 & 0xFF) << (8 * 3))
    };
}

pub trait Color {
    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;
    fn alpha(&self) -> u8;
    fn pack(&self) -> u32;
}

impl Color for RGBAColor {
    #[inline]
    fn red(&self) -> u8 {
        self.red
    }

    #[inline]
    fn green(&self) -> u8 {
        self.green
    }

    #[inline]
    fn blue(&self) -> u8 {
        self.blue
    }

    #[inline]
    fn alpha(&self) -> u8 {
        self.alpha
    }

    #[inline]
    fn pack(&self) -> u32 {
        self.into()
    }
}

impl Color for u32 {
    #[inline]
    fn red(&self) -> u8 {
        ((self & 0x000000FF) >> (8 * 0)) as u8
    }

    #[inline]
    fn green(&self) -> u8 {
        ((self & 0x0000FF00) >> (8 * 1)) as u8
    }

    #[inline]
    fn blue(&self) -> u8 {
        ((self & 0x00FF0000) >> (8 * 2)) as u8
    }

    #[inline]
    fn alpha(&self) -> u8 {
        ((self & 0xFF000000) >> (8 * 3)) as u8
    }

    #[inline]
    fn pack(&self) -> u32 {
        *self
    }
}

#[derive(Default, Debug, Clone)]
pub struct RGBAColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RGBAColor {
    pub const WHITE: RGBAColor = RGBAColor::from_rgb(255, 255, 255);
    pub const BLACK: RGBAColor = RGBAColor::from_rgb(0, 0, 0);

    pub const RED: RGBAColor = RGBAColor::from_rgb(255, 0, 0);
    pub const GREEN: RGBAColor = RGBAColor::from_rgb(0, 255, 0);
    pub const BLUE: RGBAColor = RGBAColor::from_rgb(0, 0, 255);

    pub const MAGENTA: RGBAColor = RGBAColor::from_rgb(255, 0, 255);
    pub const YELLOW: RGBAColor = RGBAColor::from_rgb(255, 255, 0);
    pub const CYAN: RGBAColor = RGBAColor::from_rgb(0, 255, 255);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: 255,
        }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }
}

impl From<Vec3> for RGBAColor {
    fn from(value: Vec3) -> Self {
        Self::from_rgb(value.x as u8, value.y as u8, value.z as u8)
    }
}

impl From<RGBAColor> for Vec3 {
    fn from(value: RGBAColor) -> Self {
        Vec3::new(value.red as f32, value.green as f32, value.blue as f32)
    }
}

impl From<RGBAColor> for u32 {
    #[inline]
    fn from(color: RGBAColor) -> Self {
        (&color).into()
    }
}

impl From<&RGBAColor> for u32 {
    #[inline]
    fn from(color: &RGBAColor) -> Self {
        ((color.red as u32 & 0xFF) << (8 * 0))
            | ((color.green as u32 & 0xFF) << (8 * 1))
            | ((color.blue as u32 & 0xFF) << (8 * 2))
            | ((color.alpha as u32 & 0xFF) << (8 * 3))
    }
}

impl From<u32> for RGBAColor {
    #[inline]
    fn from(color: u32) -> Self {
        Self {
            red: color.red(),
            green: color.green(),
            blue: color.blue(),
            alpha: color.alpha(),
        }
    }
}
