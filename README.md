# Control-the-ESP32-board-with-Rust

โปรเจกต์นี้สร้างขึ้นเพื่อควบคุมบอร์ด ESP32-S3 ด้วยภาษา Rust 
พร้อมการเชื่อมต่อ WiFi, MQTT และ OTA Firmware update อัตโนมัติจาก GitHub

## ฟีเจอร์
- ✅ ควบคุม GPIO
- ✅ เชื่อมต่อ WiFi
- ✅ OTA Firmware Update
- ✅ MQTT Subscribe เพื่อควบคุมวาล์ว

## การใช้งาน

```bash
cargo build --release
cargo espflash flash --monitor
```
## toml  สำหรับ ESP32-C3
[target.riscv32imc-esp-espidf]  
runner = "espflash flash --monitor"
