use crate::types::Color;

// System
pub const SLEEP_TIME_MS: u32 = 12;

// Network
pub const AP_SSID: &str = "Led Controller 000";
pub const HOSTNAME: &str = "led-controller-000";

// HTTP
pub const INTENSITY_REDUCTION: u8 = 3;

// LED
pub const MAX_LED_STRIP_LENGTH: usize = 101;
pub const RAINBOW_STEP: u8 = 1;
pub const RAINBOW_SATURATION: u8 = 255;
pub const RAINBOW_INTENSITY: u8 = 8;
pub const DEFAULT_COLOR: Color = Color(smart_leds::RGB::new(10, 6, 0));

// Version
pub const VERSION: &str = "v1.0.0";
pub const GIT_LINK: &str = "https://github.com/stefan-dangl/led-controller";
