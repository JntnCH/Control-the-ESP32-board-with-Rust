# Control-the-ESP32-board-with-Rust
สร้าง Rust Project ใหม่ ด้วย esp-idf-template

โปรเจกต์ควบคุมบอร์ด ESP32 ด้วยภาษา Rust พร้อมระบบ OTA (Over-the-Air)

## ⚙️ ความสามารถ

- ควบคุม GPIO (เช่น วาล์วน้ำ)
- เชื่อมต่อ WiFi
- อัปเดต firmware ผ่าน GitHub OTA
- รองรับระบบ web control หรือ MQTT

## 🚀 Build & Flash

```bash
cargo build --release
cargo espflash flash --monitor
