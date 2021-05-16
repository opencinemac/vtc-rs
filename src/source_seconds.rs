use crate::{Framerate, TimecodeParseError};
use core::result::Result;
use core::result::Result::Ok;
use num::Rational32;
use num::{FromPrimitive, Rational64};
use regex::Match;

use crate::consts::{RUNTIME_REGEX, SECONDS_PER_HOUR_I64, SECONDS_PER_MINUTE_I64};
use crate::timecode_parse::convert_tc_int;
use std::fmt::Debug;

/// The result type of [SecondsSource::to_seconds].
pub type SecondsSourceResult = Result<num::Rational64, TimecodeParseError>;

/// Types implementing this trait can be converted into the number of real-world seconds that
/// have elapsed since a timecode value of 00:00:00:00.
pub trait SecondsSource: Debug {
    fn to_seconds(&self, rate: Framerate) -> SecondsSourceResult;
}

impl SecondsSource for &dyn SecondsSource {
    fn to_seconds(&self, rate: Framerate) -> SecondsSourceResult {
        (*self).to_seconds(rate)
    }
}

impl<T> SecondsSource for &T
where
    T: SecondsSource,
{
    fn to_seconds(&self, rate: Framerate) -> SecondsSourceResult {
        (*self).to_seconds(rate)
    }
}

/// Types implementing this trait can be converted into the number of real-world seconds that have
/// elapsed since a timecode value of 00:00:00:00.
impl SecondsSource for num::Rational64 {
    fn to_seconds(&self, _: Framerate) -> SecondsSourceResult {
        Ok(*self)
    }
}

impl SecondsSource for f64 {
    fn to_seconds(&self, _: Framerate) -> SecondsSourceResult {
        // Floats are ticky, as they can often result in rational values wich try to
        // capture their imprecision using every bit available in the numerator and
        // denominator integer values.
        //
        // For this reason, we are going to first parse as a Rational32, then upgrade to
        // a Rational64. This will give operations down the line which need to multiply
        // and divide by the frame rate plenty of room to do so without running into an
        // overflow.
        let rat32 = match Rational32::from_f64(*self) {
            None => {
                return Err(TimecodeParseError::Conversion(
                    "could not convert f64 to Rational64".to_string(),
                ))
            }
            Some(parsed) => parsed,
        };

        Ok(Rational64::new(
            *rat32.numer() as i64,
            *rat32.denom() as i64,
        ))
    }
}

impl SecondsSource for f32 {
    fn to_seconds(&self, rate: Framerate) -> SecondsSourceResult {
        // Cast to an f64 then use the f64 conversion.
        f64::from(*self).to_seconds(rate)
    }
}

impl SecondsSource for &str {
    fn to_seconds(&self, rate: Framerate) -> SecondsSourceResult {
        if let Some(matched) = RUNTIME_REGEX.captures(self) {
            return parse_runtime_str(matched, rate);
        }

        Err(TimecodeParseError::UnknownStrFormat(format!(
            "{} is not a known seconds timecode format",
            self
        )))
    }
}

impl SecondsSource for String {
    fn to_seconds(&self, rate: Framerate) -> SecondsSourceResult {
        self.as_str().to_seconds(rate)
    }
}

fn parse_runtime_str(matched: regex::Captures, rate: Framerate) -> SecondsSourceResult {
    // The whole goal of this conversion will be to convert the runtime string to a rational
    // representation of it's seconds count, then use the implementation on Rational64 to finish
    // our conversion.

    // We need to figure out how many other sections were present. We'll put them into this vec.
    let mut sections: Vec<Match> = Vec::new();
    if let Some(section) = matched.name("section1") {
        sections.push(section);
    };
    if let Some(section) = matched.name("section2") {
        sections.push(section);
    };

    // Get whether this value was a negative timecode value.
    let is_negative = matched.name("negative").is_some();

    let minutes: i64 = match sections.pop() {
        None => 0,
        Some(section) => convert_tc_int(section.as_str(), "minutes")?,
    };

    let hours: i64 = match sections.pop() {
        None => 0,
        Some(section) => convert_tc_int(section.as_str(), "frames")?,
    };

    // We know this group MUST be present on a match, so we can unwrap this;
    let seconds_str = matched.name("seconds").unwrap().as_str();
    let seconds_split = seconds_str.split('.').collect::<Vec<&str>>();

    // Get the whole seconds and use it to calculate our total non-fractal seconds.
    let mut seconds = convert_tc_int(seconds_split[0], "seconds")?;
    seconds += hours * SECONDS_PER_HOUR_I64 + minutes * SECONDS_PER_MINUTE_I64;

    // Next we need to convert the fractal, which may or may not be blank, into a float. We want
    // to convert the fractal and not the whole seconds value as the smaller a float value is, the
    // more accurate it is as well.
    let maybe_fractal = seconds_split.get(1);
    let seconds_fractal_str = if let Some(seconds_fractal_str) = maybe_fractal {
        let mut fixed_fractal = "0.".to_string();
        fixed_fractal.push_str(&seconds_fractal_str);
        fixed_fractal
    } else {
        "0.0".to_string()
    };

    // Now parse the fractal as a float.
    let seconds_fractal = match seconds_fractal_str.parse::<f64>() {
        Ok(parsed) => parsed,
        Err(err) => {
            return Err(TimecodeParseError::Conversion(format!(
                "error conversion seconds of runtime to f64: {}",
                err.to_string(),
            )))
        }
    };

    // And transform it to a rational value. We are going to use a Rational32 here, then
    // cast it to a Rational64 so if we have a float which parses to a rational value
    // which would fill up the entire integer bits to be as precise as possible, we
    // don't cause an overfow when we add it to the seconds value.
    let seconds_fractal_rat32 = match Rational32::from_f64(seconds_fractal) {
        None => {
            return Err(TimecodeParseError::Conversion(
                "error conversion fractal seconds of runtime to rational".to_string(),
            ))
        }
        Some(parsed) => parsed,
    };

    let seconds_fractal_rat64 = Rational64::new(
        *seconds_fractal_rat32.numer() as i64,
        *seconds_fractal_rat32.denom() as i64,
    );

    // Which we can combine with the integer-calculated seconds to get a full rational
    // value of our seconds.
    let mut seconds_rat = Rational64::from_integer(seconds) + seconds_fractal_rat64;
    if is_negative {
        seconds_rat = -seconds_rat
    }

    // Finally, convert using the rational implementation on out seconds.
    seconds_rat.to_seconds(rate)
}
