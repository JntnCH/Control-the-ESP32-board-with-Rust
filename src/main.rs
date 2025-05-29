use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::mqtt::client::*;
use esp_idf_sys as _;
use std::{thread, time::Duration};

fn main() {
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let mut valve = PinDriver::output(peripherals.pins.gpio23).unwrap();

    // ตัวอย่าง MQTT
    let (client, mut connection) = EspMqttClient::new_with_conn("mqtt://broker.hivemq.com", &Default::default()).unwrap();
    std::thread::spawn(move || {
        while let Some(msg) = connection.next() {
            if let Ok(event) = msg {
                match event {
                    Event::Received(msg) => {
                        let data = std::str::from_utf8(msg.data()).unwrap_or("");
                        if data == "on" {
                            valve.set_high().unwrap();
                        } else if data == "off" {
                            valve.set_low().unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}