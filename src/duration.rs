use std::time::Duration;
use std::convert::TryInto;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CDuration(pub(crate) i64, pub(crate) i32);

impl From<CDuration> for Duration {
    fn from(cdur: CDuration) -> Self {
        Duration::new(
            cdur.0.try_into().unwrap(),
            (cdur.1).try_into().unwrap()
        )
    }
}

impl From<Duration> for CDuration {
    fn from(duration: Duration) -> Self {
        CDuration(
            duration.as_secs().try_into().unwrap(),
            duration.subsec_nanos().try_into().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{CDuration, Duration};

    #[test]
    fn test_into() {
        let dur = Duration::new(5, 10);
        let cdur: CDuration = dur.into();

        assert_eq!(cdur.into(): Duration, dur);
    }
}
