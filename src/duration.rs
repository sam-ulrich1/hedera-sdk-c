use std::time::Duration;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CDuration(u64, u32);

impl From<CDuration> for Duration {
    fn from(cdur: CDuration) -> Self {
        Duration::new(cdur.0, cdur.1)
    }
}

impl From<Duration> for CDuration {
    fn from(duration: Duration) -> Self {
        CDuration(duration.as_secs(), duration.subsec_nanos())
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
