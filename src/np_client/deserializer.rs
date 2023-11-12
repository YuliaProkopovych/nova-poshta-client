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

pub fn deserialize_f32_option<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: serde_json::Value = serde::de::Deserialize::deserialize(deserializer)?;

    if s.is_number() {
        return s
            .to_string()
            .parse::<f32>()
            .map(|val| Some(val))
            .map_err(|_| {
                serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
            });
    }
    if s.is_string() {
        return match s.as_str().unwrap() {
            s if s.is_empty() => Ok(None),
            s => s.parse::<f32>()
                .map(|val| Some(val))
                .map_err(|_| {
                    serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
                })
        }
        // if s.as_str().unwrap().is_empty() {
        //     return Ok(None);
        // } else {
        //     return s
        //         .as_str()
        //         .unwrap()
        //         .parse::<f32>()
        //         .map(|val| Some(val))
        //         .map_err(|_| {
        //             serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
        //         });
        // }
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
            .to_string()
            .parse::<u16>()
            .map(|val| Some(val))
            .map_err(|_| {
                serde::de::Error::unknown_variant(&s.to_string(), &["number or empty string"])
            });
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
