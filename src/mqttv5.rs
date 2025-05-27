//! MQTT V5 工具

use rumqttc::v5::Event;
use rumqttc::v5::Event::Incoming;
use rumqttc::v5::mqttbytes::v5;

/// 获取 publish 事件值
#[inline(always)]
pub fn get_publish_value(event: Event) -> Option<v5::Publish> {
    if let Incoming(packet) = event {
        if let v5::Packet::Publish(v) = packet {
            return Some(v);
        }
    }
    None
}
