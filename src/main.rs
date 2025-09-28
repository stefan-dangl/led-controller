use esp_idf_hal::delay::Delay;
use esp_idf_hal::i2c::{self, I2cDriver};
use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_hal::peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::KiloHertz;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::http::Method;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};

// #[toml_cfg::toml_config]
// pub struct Config {
//     SSID: &'static str,
//     PASSWORD: &'static str,
// }

fn wifi(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Box<EspWifi<'static>>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        anyhow::bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        log::info!("Wifi password is empty");
    }
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    log::info!("Starting wifi...");

    wifi.start()?;

    log::info!("Scanning...");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        log::info!(
            "Found configured access point {} on channel {}",
            ssid,
            ours.channel
        );
        Some(ours.channel)
    } else {
        log::info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            ssid
        );
        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid
            .try_into()
            .expect("Could not parse the given SSID into WiFi config"),
        password: pass
            .try_into()
            .expect("Could not parse the given password into WiFi config"),
        channel,
        auth_method,
        ..Default::default()
    }))?;

    log::info!("Connecting wifi...");

    wifi.connect()?;

    log::info!("Waiting for DHCP lease...");

    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    log::info!("Wifi DHCP info: {:?}", ip_info);

    Ok(Box::new(esp_wifi))
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Hallo Papa, Ich bin in einem ESP-32 Microcontroller gefangen. Hilfe!")
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

    // let app_config = CONFIG;
    // log::info!("app-config: {app_config:?}");

    log::info!("Init i2c ...");
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio0,
        peripherals.pins.gpio1,
        &i2c::config::Config::new().baudrate(KiloHertz::from(400).into()),
    );

    // Access Pixel Matrix

    log::info!("Trying to connect to wifi \"{ssid}\" ...");
    log::info!("system event loop!");

    let system_event_loop = EspSystemEventLoop::take().unwrap();
    // let nvs = EspDefaultNvsPartition::take().unwrap();

    log::info!("esp wifi new!");

    let wifi = wifi(&ssid, &password, peripherals.modem, system_event_loop);

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

    log::info!("Server awaiting request!");

    let delay = Delay::default();
    loop {
        delay.delay_ms(100);
    }
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    // 1. Create a new EspHttpConnection with default Configuration. (Check documentation)

    log::info!("HTTP Connection!");
    let connection = EspHttpConnection::new(&esp_idf_svc::http::client::Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    // // 2. Get a client using the embedded_svc Client::wrap method. (Check documentation)
    log::info!("HTTP Client!");
    let mut client = embedded_svc::http::client::Client::wrap(connection);

    // 3. Open a GET request to `url`
    let headers = [("accept", "text/plain")];
    // ANCHOR: request
    log::info!("HTTP request!");
    let request = client.request(Method::Get, url.as_ref(), &headers)?;
    // ANCHOR_END: request

    // 4. Submit the request and check the status code of the response.
    // Successful http status codes are in the 200..=299 range.
    log::info!("HTTP response!");
    let response = request.submit()?;
    let status = response.status();
    log::info!("Response code: {}\n", status);
    match status {
        200..=299 => {
            // 5. If the status is OK, read response data chunk by chunk into a buffer and print it until done.
            //
            // NB. There is no guarantee that chunks will be split at the boundaries of valid UTF-8
            // sequences (in fact it is likely that they are not) so this edge case needs to be handled.
            // However, for the purposes of clarity and brevity(?), the additional case of completely invalid
            // UTF-8 sequences will not be handled here and is left as an exercise for the reader.
            let mut buf = [0_u8; 256];
            // Offset into the buffer to indicate that there may still be
            // bytes at the beginning that have not been decoded yet
            let mut offset = 0;
            // Keep track of the total number of bytes read to print later
            let mut total = 0;
            let mut reader = response;
            loop {
                // read into the buffer starting at the offset to not overwrite
                // the incomplete UTF-8 sequence we put there earlier
                if let Ok(size) = embedded_svc::io::Read::read(&mut reader, &mut buf[offset..]) {
                    if size == 0 {
                        // It might be nice to check if we have any left over bytes here (ie. the offset > 0)
                        // as this would mean that the response ended with an invalid UTF-8 sequence, but for the
                        // purposes of this training we are assuming that the full response will be valid UTF-8
                        break;
                    }
                    // Update the total number of bytes read
                    total += size;
                    // 6. Try converting the bytes into a Rust (UTF-8) string and print it.
                    // Remember that we read into an offset and recalculate the real length
                    // of the bytes to decode.
                    let size_plus_offset = size + offset;
                    match str::from_utf8(&buf[..size_plus_offset]) {
                        Ok(text) => {
                            // buffer contains fully valid UTF-8 data,
                            // print it and reset the offset to 0.
                            print!("{}", text);
                            offset = 0;
                        }
                        Err(error) => {
                            // The buffer contains incomplete UTF-8 data, we will
                            // print the valid part, copy the invalid sequence to
                            // the beginning of the buffer and set an offset for the
                            // next read.
                            //
                            // NB. There is actually an additional case here that should be
                            // handled in a real implementation. The Utf8Error may also contain
                            // an error_len field indicating that there is actually an invalid UTF-8
                            // sequence in the middle of the buffer. Such an error would not be
                            // recoverable through our offset and copy mechanism. The result will be
                            // that the invalid sequence will be copied to the front of the buffer and
                            // eventually the buffer will be filled until no more bytes can be read when
                            // the offset == buf.len(). At this point the loop will exit without reading
                            // any more of the response.
                            let valid_up_to = error.valid_up_to();
                            unsafe {
                                // It's ok to use unsafe here as the error code already told us that
                                // the UTF-8 data up to this point is valid, so we can tell the compiler
                                // it's fine.
                                print!("{}", str::from_utf8_unchecked(&buf[..valid_up_to]));
                            }
                            buf.copy_within(valid_up_to.., 0);
                            offset = size_plus_offset - valid_up_to;
                        }
                    }
                }
            }
            log::info!("Total: {} bytes", total);
        }
        _ => anyhow::bail!("Unexpected response code: {}", status),
    }

    Ok(())
}
