use num::{traits::Inv, Rational64, Zero};

use crate::{Framerate, FramesSourceResult, TimecodeParseError};

/// convert_tc_int handles converting an int found in a string and returning an error if one
/// occurs.
pub(super) fn convert_tc_int(value: &str, section_name: &str) -> FramesSourceResult {
    return match value.parse::<i64>() {
        Ok(parsed) => Ok(parsed),
        Err(err) => Err(TimecodeParseError::Conversion(format!(
            "error converting {} to i64: {}",
            section_name, err,
        ))),
    };
}

/// takes in a seconds value and a framerate and rounds it to the nearest whole-frame.
pub(crate) fn round_seconds_to_frame(seconds: Rational64, rate: Framerate) -> Rational64 {
    if seconds % rate.playback().inv() != Rational64::zero() {
        let frames = (seconds * rate.playback()).round();
        frames / rate.playback()
    } else {
        seconds
    }
}
