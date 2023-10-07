pub mod common_date_format {
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT1: &'static str = "%Y-%m-%d %H:%M:%S";
    const FORMAT2: &'static str = "%d.%m.%Y %H:%M:%S";
    const FORMAT3: &'static str = "%d-%m-%Y %H:%M:%S";
    const FORMAT4: &'static str = "%H:%M %d.%m.%Y";
    const FORMAT5: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            None => serializer.serialize_str(""),
            Some(date) => {
                let s = format!("{}", date.format(FORMAT1));
                serializer.serialize_str(&s)
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() {
            return Ok(None);
        }

        let res = NaiveDateTime::parse_from_str(&s, FORMAT1)
            .or(NaiveDateTime::parse_from_str(&s, FORMAT2))
            .or(NaiveDateTime::parse_from_str(&s, FORMAT3))
            .or(NaiveDateTime::parse_from_str(&s, FORMAT4))
            .or(NaiveDate::parse_from_str(&s, FORMAT5)
                .map(|date| date.and_time(NaiveTime::default())))
            .map(|date| Some(date))
            .map_err(serde::de::Error::custom);
        res
    }
}

pub mod np_date_format {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%d-%m-%Y %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
