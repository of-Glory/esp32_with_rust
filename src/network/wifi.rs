use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};

// WiFi Driver를 담는 함수
pub fn wifi_driver() -> EspWifi<'static> {
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    // WiFi Config
    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: "bssm_free".into(),     // network/wifi name
            password: "bssm_free".into(), // network/wifi password
            ..Default::default()
        }))
        .unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();

    // WiFi 연결여부를 검사하는 반복문
    while !wifi_driver.is_connected().unwrap() {
        wifi_driver.get_configuration().unwrap();
    }

    println!("Should be connected now");

    wifi_driver
}
