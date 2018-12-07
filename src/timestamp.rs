use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::convert::TryInto;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CTimestamp(pub(crate) i64, pub(crate) i32);

impl From<CTimestamp> for DateTime<Utc> {
    fn from(ts: CTimestamp) -> Self {
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(
            ts.0,
            (ts.1).try_into().unwrap(),
        ))
    }
}

impl From<DateTime<Utc>> for CTimestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        CTimestamp(
            dt.timestamp(),
            dt.timestamp_subsec_nanos().try_into().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{CTimestamp, Utc, DateTime};

    #[test]
    fn test_into() {
        let dt = Utc::now();
        let ts: CTimestamp = dt.into();

        assert_eq!(ts.into(): DateTime<Utc>, dt);
    }
}
