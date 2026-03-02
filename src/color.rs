// RGBA
#[derive(Debug, Clone, Copy)]
pub struct Color(pub u32);

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        ((value.0 as u32) << 24 | (value.1 as u32) << 16 | (value.2 as u32) << 8).into()
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self(value << 8 | 0xFF)
    }
}

impl Color {
    pub fn r(&self) -> u8 {
        (self.0 >> 24) as u8
    }
    pub fn g(&self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub fn b(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub fn a(&self) -> u8 {
        self.0 as u8
    }
}
