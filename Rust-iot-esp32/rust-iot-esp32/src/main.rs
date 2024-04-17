use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    netif::{EspNetif, NetifStack},
    nvs::EspDefaultNvsPartition,
    wifi::{ClientConfiguration, Configuration, EspWifi},
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = EspWifi::new(
        EspNetif::new(NetifStack::Sta).unwrap(),
        EspSystemEventLoop::take().unwrap(),
        Some(nvs),
    )
    .unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "IFCE NASH 5G".into(),
        password: "nashifce8556".into(),
        ..Default::default()
    }))
    .unwrap();
}
