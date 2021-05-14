use crate::{FramesSourceResult, TimecodeParseError};

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
