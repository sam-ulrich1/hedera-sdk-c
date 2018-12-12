use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CTimestamp(pub i64, pub u32);

impl From<CTimestamp> for DateTime<Utc> {
    fn from(ts: CTimestamp) -> Self {
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ts.0, ts.1))
    }
}

impl From<DateTime<Utc>> for CTimestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        CTimestamp(dt.timestamp(), dt.timestamp_subsec_nanos())
    }
}

#[cfg(test)]
mod tests {
    use super::{CTimestamp, DateTime, Utc};

    #[test]
    fn test_into() {
        let dt = Utc::now();
        let ts: CTimestamp = dt.into();

        assert_eq!(ts.into(): DateTime<Utc>, dt);
    }
}
