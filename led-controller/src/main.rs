mod frontend;
mod network;
mod types;

use crate::frontend::index_html;
use crate::network::connect_to_wifi;
use crate::types::Color;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::http::Method;
use serde::Deserialize;
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
                let html = index_html();
                let mut response = request.into_ok_response()?;
                response.write_all(html.as_bytes())?;
                Ok(())
            },
        )
        .unwrap();

    let current_color_clone = current_color.clone();
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

                *current_color_clone.lock().unwrap() = Color::from(color_req.color);

                Ok(())
            },
        )
        .unwrap();

    log::info!("Server awaiting request!");

    loop {
        delay.delay_ms(100);
        let pixels = std::iter::repeat(current_color.lock().unwrap().0).take(25);
        ws2812.write_nocopy(pixels).unwrap();
    }
}
