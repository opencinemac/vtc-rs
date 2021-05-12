use num::{FromPrimitive, Rational64, Signed};
use std::str::FromStr;

use crate::errors::ParseErr;
use crate::framerate::Ntsc;

/// DROP_DIVISOR_PLAYBACK is the value that a playback rate needs to be cleanly divisible for
/// for it to be a valid NTSC playback rate.
const DROP_DIVISOR_PLAYBACK: num::Rational64 = num::Rational64::new_raw(30000, 1001);
/// DROP_DIVISOR_TIMEBASE is the value that a timebase rate needs to be cleanly divisible for
/// for it to be a valid NTSC playback rate.
const DROP_DIVISOR_TIMEBASE: num::Rational64 = num::Rational64::new_raw(30, 1);

/// validate_dropframe_ntsc validates that our ntsc and drop-frame settings are correct.
fn validate_ntsc_value(value: &num::Rational64, ntsc: Ntsc, is_base: bool) -> Result<(), String> {
    // If this value is negative, error out. We cannot have negative playback rates.
    if value.is_negative() {
        return Err("framerates cannot be negative".to_string());
    }

    // If this is not an NTSC value, return immediately.
    if !ntsc.is_ntsc() {
        return Ok(());
    }

    if is_base {
        // If this is a timebase, it needs to be a whole-number, or it is not NTSC compliant.
        if !value.is_integer() {
            return Err("ntsc timebases must be whole numbers".to_string());
        }
        // Otherwise if it is a playback speed, it must be divisible by 1001, or it is not compliant.
    } else if value.denom() != &1001 {
        return Err("ntsc framerates must be n/1001".to_string());
    }

    // If this is not a drop_frame value, we are done.
    if ntsc != Ntsc::DropFrame {
        return Ok(());
    }

    // Pick our allowed divisor based on whether this is a playback speed or timebase
    let drop_divisor = match is_base {
        true => DROP_DIVISOR_TIMEBASE,
        false => DROP_DIVISOR_PLAYBACK,
    };

    // Check that the divisor goes cleanly into the rate.
    if value % drop_divisor != num::Rational64::from_integer(0) {
        // If not get a description of the rate type for the error message.
        let rate_type = match is_base {
            true => "timebase",
            false => "playback",
        };
        let err_message = format!(
            "dropframe must have {0} divisible by {1} (multiple of 29.97)",
            rate_type, drop_divisor,
        );
        return Err(err_message);
    }

    // If we get to here, everything is a-ok!
    Ok(())
}

pub trait FramerateSource {
    fn to_playback(&self, ntsc: Ntsc, is_timebase: bool) -> Result<Rational64, ParseErr>;
}

impl FramerateSource for Rational64 {
    fn to_playback(&self, ntsc: Ntsc, is_timebase: bool) -> Result<Rational64, ParseErr> {
        validate_ntsc_value(&self, ntsc, is_timebase)?;

        let value = if is_timebase && ntsc.is_ntsc() {
            self.round() * 1000 / 1001
        } else {
            *self
        };

        Ok(value)
    }
}

impl FramerateSource for i64 {
    fn to_playback(&self, ntsc: Ntsc, is_timebase: bool) -> Result<Rational64, ParseErr> {
        let rational = num::Rational64::from_integer(*self);
        rational.to_playback(ntsc, is_timebase)
    }
}

impl FramerateSource for f64 {
    fn to_playback(&self, ntsc: Ntsc, is_timebase: bool) -> Result<Rational64, ParseErr> {
        if !ntsc.is_ntsc() {
            return Err("float values cannot be parsed if NTSC is not true".to_string());
        }
        let mut rational = match num::Rational64::from_f64(*self) {
            None => return Err("could not parse rational from f64".to_string()),
            Some(rational) => rational,
        };

        // If this is an NTSC playback speed, coerce it to the nearest correct ntsc value.
        if !is_timebase && ntsc.is_ntsc() {
            rational = rational.round() * 1000 / 1001;
        }
        rational.to_playback(ntsc, is_timebase)
    }
}

impl FramerateSource for &str {
    fn to_playback(&self, ntsc: Ntsc, is_timebase: bool) -> Result<Rational64, ParseErr> {
        if let Ok(parsed) = num::Rational64::from_str(self) {
            return parsed.to_playback(ntsc, is_timebase);
        }

        if let Ok(parsed) = i64::from_str(self) {
            return parsed.to_playback(ntsc, is_timebase);
        }

        if let Ok(parsed) = f64::from_str(self) {
            return parsed.to_playback(ntsc, is_timebase);
        }

        Err(format!(
            "could not parse '{0}' as rational, int, or float for framerate",
            self
        ))
    }
}

impl FramerateSource for String {
    fn to_playback(&self, ntsc: Ntsc, is_timebase: bool) -> Result<Rational64, ParseErr> {
        let str: &str = self.as_str();
        str.to_playback(ntsc, is_timebase)
    }
}
