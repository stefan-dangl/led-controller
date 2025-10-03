use crate::config::{MAX_LED_STRIP_LENGTH, RAINBOW_INTENSITY, RAINBOW_SATURATION, RAINBOW_STEP};
use esp_idf_hal::{gpio::OutputPin, peripheral::Peripheral, rmt::RmtChannel, sys::esp_random};
use hardware_agnostic_utils::types::Color;
use smart_leds::hsv::{hsv2rgb, Hsv};
use ws2812_esp32_rmt_driver::{Ws2812Esp32Rmt, Ws2812Esp32RmtDriverError};

pub struct Led {
    driver: Ws2812Esp32Rmt<'static>,
    hue: u8,
}

impl Led {
    pub fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'static,
        pin: impl Peripheral<P = impl OutputPin> + 'static,
    ) -> Result<Self, Ws2812Esp32RmtDriverError> {
        Ok(Self {
            driver: Ws2812Esp32Rmt::new(channel, pin)?,
            hue: unsafe { esp_random() } as u8,
        })
    }

    pub fn set_color(&mut self, color: Color) -> Result<(), Ws2812Esp32RmtDriverError> {
        let pixels = std::iter::repeat(color.0).take(MAX_LED_STRIP_LENGTH);
        self.driver.write_nocopy(pixels)
    }

    pub fn rainbow(&mut self) -> Result<(), Ws2812Esp32RmtDriverError> {
        self.hue = self.hue.wrapping_add(RAINBOW_STEP);
        let pixels = std::iter::repeat(hsv2rgb(Hsv {
            hue: self.hue,
            sat: RAINBOW_SATURATION,
            val: RAINBOW_INTENSITY,
        }))
        .take(MAX_LED_STRIP_LENGTH);
        self.driver.write_nocopy(pixels)
    }
}
