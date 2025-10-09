use std::num::ParseIntError;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Color(pub smart_leds::RGB<u8>);

impl TryFrom<String> for Color {
    type Error = ColorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            return Err(ColorError::WrongLength);
        }

        let red = u8::from_str_radix(&value[..2], 16)?;
        let green = u8::from_str_radix(&value[2..4], 16)?;
        let blue = u8::from_str_radix(&value[4..6], 16)?;
        Ok(Self(smart_leds::RGB::new(red, green, blue)))
    }
}

impl Color {
    pub fn reduce_intensity(&mut self, factor: u8) {
        self.0.r >>= factor;
        self.0.g >>= factor;
        self.0.b >>= factor;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ColorError {
    #[error("The value must consist of exactly 6 hex-symbols (rrggbb)")]
    WrongLength,

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}
