//! rand 工具

use rand::{Rng, distr::Alphanumeric};

/// 生成指定长度的随机字符串
pub fn generate_random_string(len: usize) -> String {
    let rng = rand::rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
