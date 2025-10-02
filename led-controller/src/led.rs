use esp_idf_hal::{gpio::OutputPin, peripheral::Peripheral, rmt::RmtChannel, sys::esp_random};
use smart_leds::hsv::{hsv2rgb, Hsv};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use crate::types::Color;

pub struct Led {
    driver: Ws2812Esp32Rmt<'static>,
    hue: u8,
}

impl Led {
    pub fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'static,
        pin: impl Peripheral<P = impl OutputPin> + 'static,
    ) -> Self {
        Self {
            driver: Ws2812Esp32Rmt::new(channel, pin).unwrap(),
            hue: unsafe { esp_random() } as u8,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        let pixels = std::iter::repeat(color.0).take(25);
        self.driver.write_nocopy(pixels).unwrap();
    }

    pub fn rainbow(&mut self) {
        let pixels = std::iter::repeat(hsv2rgb(Hsv {
            hue: self.hue,
            sat: 255,
            val: 8,
        }))
        .take(25);
        self.driver.write_nocopy(pixels).unwrap();
        self.hue = self.hue.wrapping_add(5);
    }
}
