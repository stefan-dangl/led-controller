mod frontend;
mod network;
mod types;
mod wifi_connection;

use crate::frontend::FRONTEND;
use crate::network::connect_to_wifi;
use crate::types::Color;
use crate::wifi_connection::CONNECTION_PAGE;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::esp_random;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::http::Method;
use serde::Deserialize;
use smart_leds::hsv::{hsv2rgb, Hsv};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

// #[toml_cfg::toml_config]
// pub struct Config {
//     SSID: &'static str,
//     PASSWORD: &'static str,
// }

#[derive(Debug, Deserialize)]
struct ColorRequest {
    color: String,
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

    let current_color = Arc::new(Mutex::new(Color::default()));
    let is_rainbow_mode = Arc::new(AtomicBool::default());

    // let app_config = CONFIG;
    // log::info!("app-config: {app_config:?}");

    log::info!("init neopixel");
    let led_pin = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

    log::info!("Start NeoPixel rainbow!");

    let delay = Delay::default();

    log::info!("Trying to connect to wifi \"{ssid}\" ...");
    log::info!("system event loop!");

    let system_event_loop = EspSystemEventLoop::take().unwrap();
    // let nvs = EspDefaultNvsPartition::take().unwrap();

    log::info!("esp wifi new!");

    let wifi = connect_to_wifi(&ssid, &password, peripherals.modem, system_event_loop);

    // get("https://espressif.com/").unwrap();

    let mut server =
        EspHttpServer::new(&esp_idf_svc::http::server::Configuration::default()).unwrap();

    // http://<sta ip>/ handler
    server
        .fn_handler(
            "/",
            Method::Get,
            |request| -> core::result::Result<(), EspIOError> {
                let mut response = request.into_ok_response()?;
                response.write_all(FRONTEND.as_bytes())?;
                Ok(())
            },
        )
        .unwrap();

    server
        .fn_handler(
            "/connection_page",
            Method::Get,
            |request| -> core::result::Result<(), EspIOError> {
                let mut response = request.into_ok_response()?;
                response.write_all(CONNECTION_PAGE.as_bytes())?;
                Ok(())
            },
        )
        .unwrap();

    let current_color_clone = current_color.clone();
    let is_rainbow_mode_clone = is_rainbow_mode.clone();
    server
        .fn_handler(
            "/set_color",
            Method::Post,
            move |mut request| -> core::result::Result<(), EspIOError> {
                let mut buf = [0; 100]; // TODO_SD: Check buffer overflow, check format
                let bytes_read = request.read(&mut buf).unwrap();
                let body = str::from_utf8(&buf[..bytes_read]).unwrap();
                let color_req: ColorRequest = serde_json::from_str(body).unwrap();

                log::info!("New Color: {}", color_req.color);

                is_rainbow_mode_clone.store(false, Ordering::SeqCst);
                *current_color_clone.lock().unwrap() = Color::from(color_req.color);

                Ok(())
            },
        )
        .unwrap();

    let is_rainbow_mode_clone = is_rainbow_mode.clone();
    server
        .fn_handler(
            "/rainbow",
            Method::Post,
            move |_| -> core::result::Result<(), EspIOError> {
                log::info!("Activate Rainbow Mode");
                is_rainbow_mode_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .unwrap();

    log::info!("Server awaiting request!");

    let mut hue = unsafe { esp_random() } as u8;

    loop {
        while is_rainbow_mode.load(Ordering::SeqCst) {
            let pixels = std::iter::repeat(hsv2rgb(Hsv {
                hue,
                sat: 255,
                val: 8,
            }))
            .take(25);
            ws2812.write_nocopy(pixels).unwrap();

            delay.delay_ms(100);

            hue = hue.wrapping_add(10);
        }
        let pixels = std::iter::repeat(current_color.lock().unwrap().0).take(25);
        ws2812.write_nocopy(pixels).unwrap();

        delay.delay_ms(100);
    }
}
