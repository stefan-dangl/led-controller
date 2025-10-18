use crate::config::HOSTNAME;
use esp_idf_hal::{peripheral::Peripheral, sys::EspError};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    ipv4::IpInfo,
    mdns::EspMdns,
    wifi::{
        AccessPointConfiguration, AccessPointInfo, AuthMethod, BlockingWifi, ClientConfiguration,
        Configuration, EspWifi,
    },
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct WiFiManager {
    wifi: Arc<Mutex<EspWifi<'static>>>,
    sysloop: EspSystemEventLoop,
    _mdns: Arc<EspMdns>,
}

impl WiFiManager {
    pub fn new(
        modem: impl Peripheral<P = Modem> + 'static,
        sysloop: EspSystemEventLoop,
    ) -> Result<Self, NetworkError> {
        let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

        let mut mdns = EspMdns::take()?;
        mdns.set_hostname(HOSTNAME)?;
        mdns.add_service(Some("Web Server"), "_http", "tcp", 80, &[("path", "/")])?;

        Ok(Self {
            wifi: Arc::new(Mutex::new(esp_wifi)),
            sysloop,
            _mdns: Arc::new(mdns),
        })
    }

    pub fn start_ap(&self, ap_ssid: &str) -> Result<(), NetworkError> {
        let mut esp_wifi = self.wifi.lock().unwrap();
        let mut wifi = BlockingWifi::wrap(&mut *esp_wifi, self.sysloop.clone())?;

        let ap_config = AccessPointConfiguration {
            ssid: ap_ssid
                .try_into()
                .map_err(|_| NetworkError::HeaplessStringConvertion)?,
            password: ""
                .try_into()
                .map_err(|_| NetworkError::HeaplessStringConvertion)?,
            channel: 1,
            auth_method: AuthMethod::None,
            ..Default::default()
        };

        let config = Configuration::Mixed(ClientConfiguration::default(), ap_config);
        wifi.set_configuration(&config)?;
        wifi.start()?;

        let ap_ip = wifi.wifi().ap_netif().get_ip_info()?;
        log::info!("AP started: SSID: {}, IP: {}", ap_ssid, ap_ip.ip);

        Ok(())
    }

    pub fn scan(&self) -> Result<Vec<AccessPointInfo>, EspError> {
        self.wifi.lock().unwrap().scan()
    }

    pub fn connect_to_wifi(
        &self,
        sta_ssid: &str,
        sta_pass: &str,
    ) -> Result<Option<IpInfo>, NetworkError> {
        let mut esp_wifi = self.wifi.lock().unwrap();
        let mut wifi = BlockingWifi::wrap(&mut *esp_wifi, self.sysloop.clone())?;

        let current_config = wifi.get_configuration()?;
        let ap_config = match current_config {
            Configuration::AccessPoint(ap) => ap,
            Configuration::Mixed(_, ap) => ap,
            _ => panic!("AP not configured"),
        };

        let mixed_config = Configuration::Mixed(
            ClientConfiguration {
                ssid: sta_ssid
                    .try_into()
                    .map_err(|_| NetworkError::HeaplessStringConvertion)?,
                password: sta_pass
                    .try_into()
                    .map_err(|_| NetworkError::HeaplessStringConvertion)?,
                ..Default::default()
            },
            ap_config,
        );

        wifi.set_configuration(&mixed_config)?;
        let result = Self::connect(&mut wifi);

        if result.is_ok() {
            Self::close_ap(&mut wifi, sta_ssid, sta_pass).expect("Failed to close AP");
        }
        result
    }

    fn close_ap(
        wifi: &mut BlockingWifi<&mut EspWifi<'static>>,
        sta_ssid: &str,
        sta_pass: &str,
    ) -> Result<(), NetworkError> {
        let client_config = Configuration::Client(ClientConfiguration {
            ssid: sta_ssid
                .try_into()
                .map_err(|_| NetworkError::HeaplessStringConvertion)?,
            password: sta_pass
                .try_into()
                .map_err(|_| NetworkError::HeaplessStringConvertion)?,
            ..Default::default()
        });
        wifi.set_configuration(&client_config)
            .map_err(NetworkError::Esp)
    }

    pub fn check_wifi_connection(&self) -> Result<Option<IpInfo>, NetworkError> {
        let mut esp_wifi = self.wifi.lock().unwrap();
        let mut wifi = BlockingWifi::wrap(&mut *esp_wifi, self.sysloop.clone())?;

        if !matches!(wifi.get_configuration()?, Configuration::Client(_)) {
            return Ok(None);
        }

        if !wifi.is_connected()? {
            log::warn!("Wifi configured but not connected. Try to reconnect");
            Self::connect(&mut wifi)
        } else {
            Ok(None)
        }
    }

    fn connect(
        wifi: &mut BlockingWifi<&mut EspWifi<'static>>,
    ) -> Result<Option<IpInfo>, NetworkError> {
        wifi.connect()?;

        if wifi.is_connected()? {
            wifi.wait_netif_up()?;
            let sta_ip = wifi.wifi().sta_netif().get_ip_info()?;
            Ok(Some(sta_ip))
        } else {
            Ok(None)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("Failed to convert to Heapless String. Input may be too long")]
    HeaplessStringConvertion,

    #[error(transparent)]
    Esp(#[from] EspError),
}
