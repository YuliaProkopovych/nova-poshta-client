pub fn deserialize_bool_from_str_num<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;

    let num = s
        .parse::<u8>()
        .map_err(|_| serde::de::Error::unknown_variant(&s, &["0", "1"]))?;
    match num {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::unknown_variant(
            &num.to_string(),
            &["0", "1"],
        )),
    }
}

pub fn deserialize_f64_option<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: serde_json::Value = serde::de::Deserialize::deserialize(deserializer)?;

    if s.is_number() {
        return Ok(s.as_f64());
    }
    if s.is_string() {
        return match s.as_str().unwrap() {
            s if s.is_empty() => Ok(None),
            s => s.parse::<f64>()
                .map(|val| Some(val))
                .map_err(|_| {
                    serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
                })
        }
    }
    if s.is_null() {
        return Ok(None);
    }
    Err(serde::de::Error::unknown_variant(
        &s.to_string(),
        &["number or empty string"],
    ))
}

pub fn deserialize_u16_option<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: serde_json::Value = serde::de::Deserialize::deserialize(deserializer)?;

    if s.is_number() {
        return s
            .as_i64()
            
            .ok_or(
                serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
            )
            .map(|val| u16::try_from(val).ok());
    }
    if s.is_string() {
        return match s.as_str().unwrap() {
            s if s.is_empty() => Ok(None),
            s => s.parse::<u16>()
                .map(|val| Some(val))
                .map_err(|_| {
                    serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
                })
        }
    }
    if s.is_null() {
        return Ok(None);
    }
    Err(serde::de::Error::unknown_variant(
        &s.to_string(),
        &["number or empty string"],
    ))
}
