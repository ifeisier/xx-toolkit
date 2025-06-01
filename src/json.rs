//! JSON 工具

use anyhow::{Result, anyhow};
use serde_json::{Map, Value};

/// 用来将 JSON 值转换成指定类型.
pub struct Convert;

/// 用来提取 JSON 值并转换成指定类型.
pub struct Extract;

impl Convert {
    /// 尝试将 JSON 值转换为 Map.
    pub fn try_into_map(value: &Value) -> Result<&Map<String, Value>> {
        value
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("字段类型错误,应为对象."))
    }
}

/// 从 JSON 对象中提取指定字段的字符串值.
pub fn get_str_field<'a>(response: &'a Value, key: &str) -> Result<&'a str> {
    response
        .get(key)
        .ok_or_else(|| anyhow!("响应数据没有{}.", key))?
        .as_str()
        .ok_or_else(|| anyhow!("响应数据{}转换错误.", key))
}

/// 从 JSON 对象中提取指定字段的 f64 值.
pub fn get_f64_field(response: &Value, key: &str) -> Result<f64> {
    response
        .get(key)
        .ok_or_else(|| anyhow!("响应数据没有{}.", key))?
        .as_f64()
        .ok_or_else(|| anyhow!("响应数据{}转换错误.", key))
}

/// 从 JSON 对象中提取指定字段的 u64 值.
pub fn get_u64_field(response: &Value, key: &str) -> Result<u64> {
    response
        .get(key)
        .ok_or_else(|| anyhow!("响应数据没有{}.", key))?
        .as_u64()
        .ok_or_else(|| anyhow!("响应数据{}转换错误.", key))
}
