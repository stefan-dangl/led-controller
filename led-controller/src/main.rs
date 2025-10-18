mod config;
mod frontend;
mod http;
mod led;
mod network;
mod types;

use crate::config::{AP_SSID, DEFAULT_COLOR, IDLE_SLEEP_TIME_MS, RAINBOW_SLEEP_TIME_MS};
use crate::http::Server;
use crate::led::Led;
use crate::network::WiFiManager;
use crate::types::Color;
use esp_idf_hal::{delay::Delay, peripherals::Peripherals};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    current_color: Arc<Mutex<Color>>,
    is_rainbow_mode: Arc<AtomicBool>,
    wifi: WiFiManager,
}

impl State {
    fn new(wifi: WiFiManager) -> Self {
        Self {
            current_color: Arc::new(Mutex::new(DEFAULT_COLOR)),
            is_rainbow_mode: Arc::new(AtomicBool::default()),
            wifi,
        }
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Init start ...");
    let peripherals = Peripherals::take().expect("Failed to access peripherals");
    let system_event_loop = EspSystemEventLoop::take().expect("Failed to access System Event Loop");

    log::info!("... Init Neopixel driver");
    let led_pin = peripherals.pins.gpio6;
    let channel = peripherals.rmt.channel0;
    let mut led = Led::new(channel, led_pin).expect("Failed to init Neopixel driver");

    log::info!("... Start WiFi-AP");
    let wifi = WiFiManager::new(peripherals.modem, system_event_loop).expect("Failed to init WiFi");
    let state = State::new(wifi);
    state
        .wifi
        .start_ap(AP_SSID)
        .expect("Failed to start WiFi-AP");

    log::info!("... Set up HTTP Server");
    let _server = Server::new(state.clone()).expect("Failed to set up HTTP Server");

    log::info!("Init done - awaiting requests");
    let delay = Delay::default();
    loop {
        if state.is_rainbow_mode.load(Ordering::SeqCst) {
            if let Err(err) = led.rainbow() {
                log::error!("Failed to set rainbow color: {err}")
            }
            delay.delay_ms(RAINBOW_SLEEP_TIME_MS);
        } else {
            #[warn(clippy::collapsible_else_if)]
            if let Err(err) = led.set_color(*state.current_color.lock().unwrap()) {
                log::error!("Failed to set color: {err}")
            }
            delay.delay_ms(IDLE_SLEEP_TIME_MS);
        }
    }
}
