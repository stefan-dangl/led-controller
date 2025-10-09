use crate::types::Color;
use crate::{
    config::INTENSITY_REDUCTION,
    frontend::{color_panel::color_panel, index::index, wifi_connection::connection_page},
    State,
};
use esp_idf_hal::{
    io::{EspIOError, Write},
    sys::EspError,
};
use esp_idf_svc::http::{
    server::{Configuration, EspHttpConnection, EspHttpServer, Request},
    Method,
};
use serde::{Deserialize, Serialize};
use std::{str::Utf8Error, sync::atomic::Ordering};

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
    pub fn new(state: State) -> Result<Self, EspIOError> {
        let server = EspHttpServer::new(&Configuration::default())?;
        let mut this = Self(server);

        this.get_index()?;
        this.get_color_panel()?;
        this.get_networks(state.clone())?;
        this.connect_to_wifi(state.clone())?;
        this.set_color(state.clone())?;
        this.rainbow_mode(state)?;

        Ok(this)
    }

    fn get_index(&mut self) -> Result<(), EspError> {
        self.0
            .fn_handler("/", Method::Get, |request| -> Result<(), HttpError> {
                let mut response = request.into_ok_response()?;
                response.write_all(index().as_bytes())?;
                Ok(())
            })?;
        Ok(())
    }

    fn get_color_panel(&mut self) -> Result<(), EspError> {
        self.0.fn_handler(
            "/color_panel",
            Method::Get,
            |request| -> Result<(), HttpError> {
                let mut response = request.into_ok_response()?;
                response.write_all(color_panel().as_bytes())?;
                Ok(())
            },
        )?;
        Ok(())
    }

    fn get_networks(&mut self, state: State) -> Result<(), EspError> {
        self.0.fn_handler(
            "/connection_page",
            Method::Get,
            move |request| -> Result<(), HttpError> {
                log::info!("Scan networks ...");

                match state.wifi.scan() {
                    Ok(ap_infos) => {
                        let mut response = request.into_ok_response()?;
                        response.write_all(connection_page(&ap_infos).as_bytes())?;
                    }
                    Err(err) => {
                        let error_message = format!("Failed to scan networks: {err}");
                        log::error!("{error_message}");
                        let mut response = request.into_response(500, Some(&error_message), &[])?;
                        response.write_all(error_message.as_bytes())?;
                    }
                }
                Ok(())
            },
        )?;
        Ok(())
    }

    fn connect_to_wifi(&mut self, state: State) -> Result<(), EspError> {
        self.0.fn_handler(
            "/connect_to_wifi",
            Method::Post,
            move |mut request| -> Result<(), HttpError> {
                let body = read_body(&mut request)?;
                let connection_req: ConnectionRequest = serde_json::from_str(&body)?;

                log::info!("Try connecting to WiFi {}", connection_req.ssid,);
                match state
                    .wifi
                    .connect_to_wifi(&connection_req.ssid, &connection_req.password)
                {
                    Ok(Some(sta_ip)) => {
                        log::info!(
                            "Successfully connected to WiFi {}. IP-Address: {sta_ip:?}",
                            connection_req.ssid
                        );
                        let response_data = ConnectionResponse {
                            ip_address: sta_ip.ip.to_string(),
                        };
                        let mut response = request.into_ok_response()?;
                        let json_bytes = serde_json::to_vec(&response_data)?;
                        response.write_all(&json_bytes)?;
                    }
                    others => {
                        let error_message = if let Err(err) = others {
                            format!("Failed to connect to WiFi: {err}")
                        } else {
                            "Failed to connect to WiFi".to_owned()
                        };
                        log::error!("{error_message}");
                        let mut response = request.into_response(500, Some(&error_message), &[])?;
                        response.write_all(error_message.as_bytes())?;
                    }
                }
                Ok(())
            },
        )?;
        Ok(())
    }

    fn set_color(&mut self, state: State) -> Result<(), EspError> {
        self.0.fn_handler(
            "/set_color",
            Method::Post,
            move |mut request| -> Result<(), HttpError> {
                let body = read_body(&mut request)?;
                let color_req: ColorRequest = serde_json::from_str(&body)?;
                log::info!("New color: {}", color_req.color);

                state.is_rainbow_mode.store(false, Ordering::SeqCst);
                match Color::try_from(color_req.color) {
                    Ok(mut color) => {
                        color.reduce_intensity(INTENSITY_REDUCTION);
                        *state.current_color.lock().unwrap() = color;
                        Ok(())
                    }
                    Err(err) => {
                        let error_message = format!("Invalid color Format: {err}");
                        log::error!("{error_message}");
                        let mut response = request.into_response(400, Some(&error_message), &[])?;
                        response.write_all(error_message.as_bytes())?;
                        Ok(())
                    }
                }
            },
        )?;
        Ok(())
    }

    fn rainbow_mode(&mut self, state: State) -> Result<(), EspError> {
        self.0.fn_handler(
            "/rainbow",
            Method::Post,
            move |_| -> Result<(), EspIOError> {
                state.is_rainbow_mode.store(true, Ordering::SeqCst);
                Ok(())
            },
        )?;
        Ok(())
    }
}

fn read_body(request: &mut Request<&mut EspHttpConnection<'_>>) -> Result<String, HttpError> {
    const MAX_REQUEST_LENGTH: usize = 100;
    let mut buf = [0; MAX_REQUEST_LENGTH];
    let bytes_read = request.read(&mut buf)?;
    let string_slice = std::str::from_utf8(&buf[..bytes_read])?;
    Ok(string_slice.to_string())
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    EspIo(#[from] EspIOError),
}
