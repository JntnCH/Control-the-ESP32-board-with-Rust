// ฟังก์ชันควบคุมวาล์ว - ประกาศก่อนเพื่อให้แน่ใจว่าพร้อมใช้งานทันที
function controlValve(valveNumber, action) {
  const statusElement = document.getElementById(`status-${valveNumber}`);
  
  // อัปเดต UI
  if (action === 'open') {
    statusElement.textContent = 'เปิด';
  } else if (action === 'closed') {
    statusElement.textContent = 'ปิด';
  }
  
  // ส่งคำสั่งผ่าน MQTT
  if (typeof client !== 'undefined') {
    const topic = `valve/${valveNumber}/control`;
    client.publish(topic, action, { qos: 0 }, (err) => {
      if (err) {
        statusElement.textContent = "ส่งคำสั่งไม่ได้";
        console.error(`ไม่สามารถส่งคำสั่งไปยังวาล์ว ${valveNumber}: ${err}`);
      } else {
        console.log(`ควบคุมวาล์ว ${valveNumber}: ${action}`);
      }
    });
  } else {
    console.log(`MQTT client ยังไม่พร้อม, ควบคุมวาล์ว ${valveNumber}: ${action}`);
  }
}

// รอให้ DOM โหลดเสร็จก่อนทำงานกับ MQTT
document.addEventListener('DOMContentLoaded', function() {
  // เชื่อมต่อ MQTT broker
  try {
    const MQTT_BROKER = "ws://broker.hivemq.com:8000/mqtt"; // ใช้ WebSocket สำหรับ browser
    const client = mqtt.connect(MQTT_BROKER);

    // เมื่อเชื่อมต่อสำเร็จ
    client.on("connect", () => {
      console.log("Connected to MQTT broker");
      // Subscribe เพื่อรับสถานะจาก ESP32
      for (let valveId = 1; valveId <= 6; valveId++) {
        client.subscribe(`valve/${valveId}/status`);
      }
    });

    // รับสถานะจาก ESP32
    client.on("message", (topic, message) => {
      const valveId = parseInt(topic.split("/")[1]);
      const status = message.toString();
      const statusElement = document.getElementById(`status-${valveId}`);
      statusElement.textContent = status === "open" ? "เปิด" : "ปิด";
    });
  } catch (e) {
    console.error("ไม่สามารถเชื่อมต่อ MQTT:", e);
  }
});

// ตัวแปรสำหรับตรวจจับทิศทางการ scroll
let lastScrollTop = 0;

// เพิ่มเอฟเฟกต์ auto-hide เมื่อเลื่อนหน้าจอ
window.addEventListener('scroll', function() {
  const nav = document.querySelector('nav');
  const currentScrollTop = window.pageYOffset || document.documentElement.scrollTop;
  
  // ตรวจสอบทิศทางการ scroll
  if (currentScrollTop > lastScrollTop) {
    // เลื่อนลง - แสดงกล่อง
    nav.classList.remove('hide');
  } else {
    // เลื่อนขึ้น - ซ่อนกล่อง
    nav.classList.add('hide');
  }
  
  // อัปเดตตำแหน่ง scroll ล่าสุด
  lastScrollTop = currentScrollTop <= 0 ? 0 : currentScrollTop;
});
