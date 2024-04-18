use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    netif::{EspNetif, NetifStack},
    nvs::EspDefaultNvsPartition,
    wifi::{ClientConfiguration, Configuration, EspWifi},
};
use heapless::String;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let wifi_ssid: String<32> = String::try_from("IFCE NASH 5G").expect("INVALID SSID");
    let wifi_pass: String<64> = String::try_from("nashifce8556").expect("INVALID PASSWORD");

    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = EspWifi::new(
        EspNetif::new(NetifStack::Sta).unwrap(),
        EspSystemEventLoop::take().unwrap(),
        Some(nvs),
    )
    .unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: wifi_ssid,
        password: wifi_pass,
        ..Default::default()
    }))
    .unwrap();
}
