use crate::{Framerate, TimecodeParseError};
use std::convert::TryFrom;
use std::fmt::Debug;

/// The result type of [PremiereTicksSource::to_frames].
pub type PremiereTicksSourceResult = Result<i64, TimecodeParseError>;

/// Types implementing this trait can be converted into the number of Adobe Premiere Pro Ticks that
/// have elapsed since a timecode value of 00:00:00:00.
pub trait PremiereTicksSource: Debug {
    fn to_ticks(&self, rate: Framerate) -> PremiereTicksSourceResult;
}

impl<T> PremiereTicksSource for &T
where
    T: PremiereTicksSource,
{
    fn to_ticks(&self, rate: Framerate) -> PremiereTicksSourceResult {
        (*self).to_ticks(rate)
    }
}

impl PremiereTicksSource for &dyn PremiereTicksSource {
    fn to_ticks(&self, rate: Framerate) -> PremiereTicksSourceResult {
        (*self).to_ticks(rate)
    }
}

impl PremiereTicksSource for i64 {
    fn to_ticks(&self, _: Framerate) -> PremiereTicksSourceResult {
        Ok(*self)
    }
}

impl PremiereTicksSource for u64 {
    fn to_ticks(&self, _: Framerate) -> PremiereTicksSourceResult {
        let i64_val = match i64::try_from(*self) {
            Ok(converted) => converted,
            Err(err) => {
                return Err(TimecodeParseError::Conversion(format!(
                    "error converting u64 to i64 : {}",
                    err.to_string()
                )))
            }
        };

        Ok(i64_val)
    }
}
