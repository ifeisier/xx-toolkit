//! IP 相关的工具

use super::reqwest::get;
use crate::json::{get_f64_field, get_str_field};
use anyhow::Result;

/// 坐标
#[derive(Debug, Default)]
pub struct Coordinates {
    /// IP 地址
    pub addr: String,
    /// 纬度
    pub lat: Option<f64>,
    /// 经度
    pub lon: Option<f64>,
}

/// 通过IP获取地址和经纬度
pub async fn get_coordinates_from_ip(ip_address: &str) -> Result<Coordinates> {
    let mut coordinates = Coordinates::default();

    // 获取经纬度
    let url = format!(
        "http://ip-api.com/json/{}?fields=61439&lang=zh-CN",
        ip_address
    );
    let response = get(&url).await?;

    let text = String::from_utf8_lossy(&response);
    let response = serde_json::from_str::<serde_json::Value>(&text.into_owned())?;
    let status = get_str_field(&response, "status")?;
    if status == "fail" {
        coordinates.addr = "局域网".to_owned();
        return Ok(coordinates);
    }

    let country = get_str_field(&response, "country")?;
    let region_name = get_str_field(&response, "regionName")?;
    let city = get_str_field(&response, "city")?;
    coordinates.addr = format!("{} {} {}", country, region_name, city);

    coordinates.lat = Some(get_f64_field(&response, "lat")?);
    coordinates.lon = Some(get_f64_field(&response, "lon")?);

    Ok(coordinates)
}
