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

#[cfg(test)]
mod test {
    use super::*;

    #[test_case::test_case(
        "804020",
        Color(smart_leds::RGB::new(0x80, 0x40, 0x20)),
        Color(smart_leds::RGB::new(0x10, 0x08, 0x04))
    )]
    #[test_case::test_case(
        "000000",
        Color(smart_leds::RGB::new(0x00, 0x00, 0x00)),
        Color(smart_leds::RGB::new(0x00, 0x00, 0x00))
    )]
    fn test_reduce_intensity(input: &str, input_color: Color, reduced_color: Color) {
        let mut color = Color::try_from(input.to_owned()).unwrap();
        assert_eq!(color, input_color);
        color.reduce_intensity(3);
        assert_eq!(color, reduced_color);
    }

    #[test_case::test_case("12345")]
    #[test_case::test_case("1234567")]
    #[test_case::test_case("uvw!@?")]
    fn test_try_from_string_failed(input: &str) {
        assert!(Color::try_from(input.to_owned()).is_err());
    }
}
