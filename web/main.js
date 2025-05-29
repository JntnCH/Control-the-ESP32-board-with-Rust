const ESP32_IP = 'http://192.168.1.123';

function controlValve(state) {
  fetch(`${ESP32_IP}/valve/${state}`)
    .then(res => {
      document.getElementById('status').textContent =
        state === 'on' ? 'วาล์วเปิดแล้ว' : 'วาล์วปิดแล้ว';
    })
    .catch(() => {
      document.getElementById('status').textContent = 'เชื่อมต่อ ESP32 ไม่ได้';
    });
}