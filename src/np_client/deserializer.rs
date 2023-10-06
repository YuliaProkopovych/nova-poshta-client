pub fn deserialize_bool_from_str_num<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;

    let num = s.parse::<u8>()
        .map_err(|_| serde::de::Error::unknown_variant(&s, &["0", "1"]))?;
    match num {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::unknown_variant(&num.to_string(), &["0", "1"])),
     }
}