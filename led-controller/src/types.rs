#[derive(Default, Clone, Copy)]
pub struct Color(pub smart_leds::RGB<u8>);

impl From<String> for Color {
    fn from(value: String) -> Self {
        let red = u8::from_str_radix(&value[..2], 16).unwrap();
        let green = u8::from_str_radix(&value[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&value[4..6], 16).unwrap();
        Self(smart_leds::RGB::new(red, green, blue))
    }
}

impl Color {
    pub fn reduce_intensity(&mut self, factor: u8) {
        self.0.r >>= factor;
        self.0.g >>= factor;
        self.0.b >>= factor;
    }
}
