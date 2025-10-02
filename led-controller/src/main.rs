mod frontend;
mod http;
mod led;
mod network;
mod types;

use crate::http::Server;
use crate::led::Led;
use crate::network::WiFiManager;
use crate::types::Color;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::peripherals::Peripherals;
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
            current_color: Arc::new(Mutex::new(Color::default())),
            is_rainbow_mode: Arc::new(AtomicBool::default()),
            wifi,
        }
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Set during combilation, or better, try to configure it via serial interface. nvs meight use it for automatic connecting to wifi:
    // Starting MCU -> Try connect to Wifi. If failed, wait for Serial Input defining SSID and Password
    // let ssid = std::env::var("SSID").expect("Please set SSID ENV for Wifi connection");
    // let password = std::env::var("PASSWORD").expect("Please set PASSWORD ENV for Wifi connection");
    let ssid = "xxx".to_owned();
    let password = "xxx".to_owned();

    let peripherals = Peripherals::take().expect("Failed to take peripherals");

    log::info!("init neopixel");
    let led_pin = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut led = Led::new(channel, led_pin);

    log::info!("Start NeoPixel rainbow!");

    let delay = Delay::default();

    log::info!("Trying to connect to wifi \"{ssid}\" ...");
    log::info!("system event loop!");

    let system_event_loop = EspSystemEventLoop::take().unwrap();
    // let nvs = EspDefaultNvsPartition::take().unwrap();

    log::info!("Start AP!");
    let wifi = network::WiFiManager::new(peripherals.modem, system_event_loop)
        .expect("Failed to create wifi struct");

    let state = State::new(wifi);
    state
        .wifi
        .start_ap_only("!!! MY SUPER COOL AP")
        .expect("Failed to start AP");

    let server = Server::new(state.clone());

    log::info!("Server awaiting request!");

    loop {
        while state.is_rainbow_mode.load(Ordering::SeqCst) {
            led.rainbow();
            delay.delay_ms(100);
        }
        led.set_color(state.current_color.lock().unwrap().clone());
        delay.delay_ms(100);
    }
}
