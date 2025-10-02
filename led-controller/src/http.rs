use std::sync::atomic::Ordering;

use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_svc::http::{server::EspHttpServer, Method};
use serde::{Deserialize, Serialize};

use crate::{
    frontend::{color_panel, index, wifi_connection::connection_page},
    types::Color,
    State,
};

#[derive(Debug, Deserialize)]
struct ColorRequest {
    color: String,
}

#[derive(Debug, Deserialize)]
struct ConnectionRequest {
    ssid: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct ConnectionResponse {
    ip_address: String,
}

pub struct Server(EspHttpServer<'static>);

impl Server {
    pub fn new(state: State) -> Self {
        let server =
            EspHttpServer::new(&esp_idf_svc::http::server::Configuration::default()).unwrap();
        let mut this = Self(server);

        this.get_index();
        this.get_color_panel();
        this.get_networks(state.clone());
        this.connect_to_wifi(state.clone());
        this.set_color(state.clone());
        this.rainbow_mode(state);

        this
    }

    fn get_index(&mut self) {
        self.0
            .fn_handler(
                "/",
                Method::Get,
                |request| -> core::result::Result<(), EspIOError> {
                    // TODO_SD: Already connected -> color_panel
                    log::info!("Index endpoint called");

                    let mut response = request.into_ok_response()?;
                    response.write_all(index::HTML.as_bytes())?;
                    Ok(())
                },
            )
            .unwrap();
    }

    fn get_color_panel(&mut self) {
        self.0
            .fn_handler(
                "/color_panel",
                Method::Get,
                |request| -> core::result::Result<(), EspIOError> {
                    let mut response = request.into_ok_response()?;
                    response.write_all(color_panel::HTML.as_bytes())?;
                    Ok(())
                },
            )
            .unwrap();
    }

    fn get_networks(&mut self, state: State) {
        self.0
            .fn_handler(
                "/connection_page",
                Method::Get,
                move |request| -> core::result::Result<(), EspIOError> {
                    log::info!("Scan networks ...");

                    let ap_infos = match state.wifi.scan() {
                        Ok(ap_infos) => ap_infos,
                        Err(err) => {
                            let error_message = format!("Failed to scan networks: {err}");
                            log::error!("{error_message}");
                            let response = request.into_response(500, Some(&error_message), &[]); // TODO_SD: Used?
                            return Err(err.into());
                        }
                    };

                    let mut response = request.into_ok_response()?;
                    response.write_all(connection_page(&ap_infos).as_bytes())?;
                    Ok(())
                },
            )
            .unwrap();
    }

    fn connect_to_wifi(&mut self, state: State) {
        self.0
            .fn_handler(
                "/connect_to_wifi",
                Method::Post,
                move |mut request| -> core::result::Result<(), EspIOError> {
                    log::info!("!!! Connect to WIFI called");
                    let mut buf = [0; 100]; // TODO_SD: Check buffer overflow, check format
                    let bytes_read = request.read(&mut buf).unwrap();
                    let body = str::from_utf8(&buf[..bytes_read]).unwrap();
                    let connection_req: ConnectionRequest = serde_json::from_str(body).unwrap();

                    log::info!(
                        "Wants to connect to: {} with password {}",
                        connection_req.ssid,
                        connection_req.password
                    );

                    match state
                        .wifi
                        .connect_to_wifi(&connection_req.ssid, &connection_req.password)
                    {
                        Ok(Some(sta_ip)) => {
                            log::info!("Successfully connected to WiFi. IP-Address: {sta_ip:?}");
                            let response_data = ConnectionResponse {
                                ip_address: sta_ip.ip.to_string(),
                            };
                            let mut response = request.into_ok_response()?;
                            let json_bytes = serde_json::to_vec(&response_data)
                                .map_err(|e| {
                                    log::error!("Failed to serialize response data: {}", e);
                                    e
                                })
                                .unwrap();
                            response.write_all(&json_bytes).map_err(|e| {
                                log::error!("Failed to write response: {}", e);
                                e
                            })?;
                            Ok(())
                        }
                        Ok(None) => {
                            let error_message = format!("Failed to connect to WiFi");
                            log::error!("{error_message}");
                            let response = request.into_response(500, Some(&error_message), &[]); // TODO_SD: Used?
                            Ok(()) // TODO_SD: Return well fitting error
                        }
                        Err(err) => {
                            let error_message = format!("Failed to connect to WiFi: {err}");
                            log::error!("{error_message}");
                            let response = request.into_response(500, Some(&error_message), &[]); // TODO_SD: Used?
                            Ok(()) // TODO_SD: Return well fitting error
                        }
                    }
                },
            )
            .unwrap();
    }

    fn set_color(&mut self, state: State) {
        self.0
            .fn_handler(
                "/set_color",
                Method::Post,
                move |mut request| -> core::result::Result<(), EspIOError> {
                    log::info!("!!! Set color called");

                    let mut buf = [0; 100]; // TODO_SD: Check buffer overflow, check format
                    let bytes_read = request.read(&mut buf).unwrap();
                    let body = str::from_utf8(&buf[..bytes_read]).unwrap();
                    let color_req: ColorRequest = serde_json::from_str(body).unwrap();

                    log::info!("New Color: {}", color_req.color);

                    state.is_rainbow_mode.store(false, Ordering::SeqCst);
                    *state.current_color.lock().unwrap() = Color::from(color_req.color);

                    Ok(())
                },
            )
            .unwrap();
    }

    fn rainbow_mode(&mut self, state: State) {
        self.0
            .fn_handler(
                "/rainbow",
                Method::Post,
                move |_| -> core::result::Result<(), EspIOError> {
                    log::info!("Activate Rainbow Mode");
                    state.is_rainbow_mode.store(true, Ordering::SeqCst);
                    Ok(())
                },
            )
            .unwrap();
    }
}
