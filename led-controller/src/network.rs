use std::sync::{Arc, Mutex};

use esp_idf_hal::{peripheral, sys::EspError};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    ipv4::IpInfo,
    wifi::{
        AccessPointConfiguration, AccessPointInfo, AuthMethod, BlockingWifi, ClientConfiguration,
        Configuration, EspWifi,
    },
};

#[derive(Clone)]
pub struct WiFiManager {
    wifi: Arc<Mutex<EspWifi<'static>>>,
    sysloop: EspSystemEventLoop,
}

impl WiFiManager {
    pub fn new(
        modem: impl peripheral::Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
        sysloop: EspSystemEventLoop,
    ) -> anyhow::Result<Self> {
        let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

        Ok(Self {
            wifi: Arc::new(Mutex::new(esp_wifi)),
            sysloop,
        })
    }

    pub fn start_ap_only(&self, ap_ssid: &str) -> anyhow::Result<()> {
        let mut esp_wifi = self.wifi.lock().unwrap();
        let mut wifi = BlockingWifi::wrap(&mut *esp_wifi, self.sysloop.clone())?;

        log::info!("Define config");
        let ap_config = AccessPointConfiguration {
            ssid: ap_ssid.try_into().unwrap(),
            password: "".try_into().unwrap(),
            channel: 1,
            auth_method: AuthMethod::None,
            ..Default::default()
        };

        // Use mixed mode instead of pure AP mode
        let config = Configuration::Mixed(ClientConfiguration::default(), ap_config);
        log::info!("Set config");
        wifi.set_configuration(&config)?;
        log::info!("start");
        wifi.start()?;
        // log::info!("wait netif up");
        // wifi.wait_netif_up()?;

        log::info!("get IP info");

        let ap_ip = wifi.wifi().ap_netif().get_ip_info()?;
        log::info!("AP started: {} - IP: {}", ap_ssid, ap_ip.ip);

        Ok(())
    }

    pub fn scan(&self) -> Result<Vec<AccessPointInfo>, EspError> {
        log::info!("!!! Scan network");
        self.wifi.lock().unwrap().scan()
    }

    pub fn connect_to_wifi(
        &self,
        sta_ssid: &str,
        sta_pass: &str,
    ) -> anyhow::Result<Option<IpInfo>> {
        let mut esp_wifi = self.wifi.lock().unwrap();
        // TODO_SD: Deadlock?
        let mut wifi = BlockingWifi::wrap(&mut *esp_wifi, self.sysloop.clone())?;

        // Get current AP configuration to maintain it
        let current_config = wifi.get_configuration()?;
        let ap_config = match current_config {
            Configuration::AccessPoint(ap) => ap,
            Configuration::Mixed(_, ap) => ap,
            _ => anyhow::bail!("AP not configured"),
        };

        // Switch to mixed mode
        let mixed_config = Configuration::Mixed(
            ClientConfiguration {
                ssid: sta_ssid.try_into().unwrap(), // TODO_SD: Validate strings without unwrap
                password: sta_pass.try_into().unwrap(),
                ..Default::default()
            },
            ap_config,
        );

        wifi.set_configuration(&mixed_config)?;

        log::info!("Connecting to WiFi: {}", sta_ssid);
        wifi.connect()?;

        if wifi.is_connected()? {
            wifi.wait_netif_up()?;
            let sta_ip = wifi.wifi().sta_netif().get_ip_info()?;
            log::info!("Connected to WiFi! STA IP: {}", sta_ip.ip);
            Ok(Some(sta_ip))
        } else {
            log::warn!("Failed to connect to WiFi");
            Ok(None)
        }
    }
}
