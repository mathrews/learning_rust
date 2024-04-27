use anyhow;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripherals::Peripherals,
    http::{
        client::{Configuration as HttpConfig, EspHttpConnection, Request},
        Method,
    },
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};
use heapless::String;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // Configure Wifi
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi_ssid: String<32> = String::try_from("IFCE NASH 5G").expect("INVALID SSID");
    let wifi_pass: String<64> = String::try_from("nashifce8556").expect("INVALID PASSWORD");

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: wifi_ssid,
        bssid: None,
        auth_method: AuthMethod::None,
        password: wifi_pass,
        channel: None,
    }))?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    // Print Out Wifi Connection Configuration
    while !wifi.is_connected().unwrap() {
        // Get and print connection Configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }

    println!("Wifi Connected, Initializing HTTP client");

    // HTTP Configuration
    // Create HTTPS Connection Handle
    let httpconnection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    // Create HTTPS Client
    let mut http_client = Request::wrap(httpconnection);

    // HTTP Request Submission
    // Define URL
    let url = "https://httpbin.org/get";

    let request = http_client.connection().initiate_request(
        Method::Get,
        url.as_ref(),
        &[("accept", "text/plain")],
    );

    Ok(())
}
