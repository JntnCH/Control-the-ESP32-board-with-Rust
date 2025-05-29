use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::http::client::*;
use esp_idf_sys as _;
use std::{thread, time::Duration, str, io::Write};

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut valve = PinDriver::output(peripherals.pins.gpio23).unwrap();

    // MQTT setup
    let mqtt_url = "mqtt://broker.hivemq.com";
    let (mut client, mut conn) = EspMqttClient::new_with_conn(mqtt_url, &Default::default()).unwrap();

    // Subscribe to control topic
    client.subscribe("valve/control", QoS::AtMostOnce).unwrap();

    // Spawn MQTT listener
    std::thread::spawn(move || {
        while let Some(msg) = conn.next() {
            if let Ok(Event::Received(msg)) = msg {
                let data = std::str::from_utf8(msg.data()).unwrap_or("");
                match data {
                    "on" => {
                        valve.set_high().unwrap();
                        client.publish("valve/status", QoS::AtMostOnce, false, "on").unwrap();
                    },
                    "off" => {
                        valve.set_low().unwrap();
                        client.publish("valve/status", QoS::AtMostOnce, false, "off").unwrap();
                    },
                    _ => {}
                }
            }
        }
    });

    // Optional: OTA version check
    check_version_and_update("1.0.0");

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn check_version_and_update(current_version: &str) {
    let version_url = "https://your-server.com/version.txt"; // เปลี่ยนเป็น URL จริงของคุณ
    let bin_url = "https://your-server.com/firmware.bin";    // OTA firmware binary URL

    let client = EspHttpConnection::new(&Default::default()).unwrap();
    let mut res = EspHttpClient::new(client).get(version_url).unwrap().submit().unwrap();
    let mut buf = [0u8; 32];
    let len = res.read(&mut buf).unwrap_or(0);
    let remote_version = str::from_utf8(&buf[..len]).unwrap_or("").trim();

    if remote_version != current_version {
        perform_ota(bin_url);
    }
}

fn perform_ota(bin_url: &str) {
    let conn = EspHttpConnection::new(&Default::default()).unwrap();
    let mut res = EspHttpClient::new(conn).get(bin_url).unwrap().submit().unwrap();
    let mut ota = esp_idf_svc::ota::EspOtaUpdate::begin().unwrap();

    let mut buf = [0u8; 1024];
    while let Ok(read) = res.read(&mut buf) {
        if read == 0 { break; }
        ota.write_all(&buf[..read]).unwrap();
    }

    ota.finalize().unwrap();
    esp_idf_sys::esp_restart();
}

// Optional: Firebase Integration (ใช้ REST API)
/*
fn read_valve_status_from_firebase() -> Option<bool> {
    let client = EspHttpConnection::new(&Default::default()).ok()?;
    let mut response = EspHttpClient::new(client)
        .get("https://<PROJECT_ID>.firebaseio.com/valve/status.json")
        .unwrap()
        .submit()
        .unwrap();

    let mut buf = [0u8; 64];
    let len = response.read(&mut buf).unwrap_or(0);
    let data = str::from_utf8(&buf[..len]).unwrap_or("false");
    Some(data.trim() == "true")
}
*/