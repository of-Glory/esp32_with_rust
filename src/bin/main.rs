use esp_idf_sys as _;
use std::{thread::sleep, time::Duration};
use use_std_esp32_rust::{network::wifi_driver, res::response};

fn main() {
    // Entry point
    esp_idf_sys::link_patches(); // Needed for esp32-rs

    println!("Entered Main function!");
    let wifi = wifi_driver();

    loop {
        println!("IP info: {:?}", wifi.sta_netif().get_ip_info().unwrap());
        response();
        sleep(Duration::new(10, 0));
    }
}
