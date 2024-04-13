use std::collections::HashMap;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

lazy_static! {
    static ref UNIT_MAP: HashMap<&'static str, f64> = {
        let mut map = HashMap::new();
        map.insert("B", 1f64);
        map.insert("kB", 1000f64);
        map.insert("MB", 1000f64 * 1000f64);
        map.insert("GB", 1000f64 * 1000f64 * 1000f64);
        map.insert("TB", 1000f64 * 1000f64 * 1000f64 * 1000f64);
        map
    };
}

pub fn convert_to_bytes(value: f64, unit: String) -> Result<f64> {
    let Some(conversion_rate) = UNIT_MAP.get(unit.as_str()) else {
        return Err(anyhow!("Couldn't convert unit '{}' to bytes, that was weird..", unit));
    };

    let result = conversion_rate * value;
    Ok(result)
}
