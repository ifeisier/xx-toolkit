//! JSON 工具

use anyhow::{Result, anyhow};

/// 从 JSON 对象中提取指定字段的字符串值.
pub fn get_str_field<'a>(response: &'a serde_json::Value, key: &str) -> Result<&'a str> {
    response
        .get(key)
        .ok_or_else(|| anyhow!("响应数据没有{}.", key))?
        .as_str()
        .ok_or_else(|| anyhow!("响应数据{}转换错误.", key))
}

/// 从 JSON 对象中提取指定字段的 f64 值.
pub fn get_f64_field(response: &serde_json::Value, key: &str) -> Result<f64> {
    response
        .get(key)
        .ok_or_else(|| anyhow!("响应数据没有{}.", key))?
        .as_f64()
        .ok_or_else(|| anyhow!("响应数据{}转换错误.", key))
}
