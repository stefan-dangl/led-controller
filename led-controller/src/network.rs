use esp_idf_hal::{peripheral::Peripheral, sys::EspError};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    ipv4::IpInfo,
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
}

impl WiFiManager {
    pub fn new(
        modem: impl Peripheral<P = Modem> + 'static,
        sysloop: EspSystemEventLoop,
    ) -> Result<Self, NetworkError> {
        let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

        Ok(Self {
            wifi: Arc::new(Mutex::new(esp_wifi)),
            sysloop,
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
