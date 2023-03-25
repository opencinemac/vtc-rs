use lazy_static::lazy_static;
use num::rational::Ratio;
use num::Rational64;
use regex::Regex;

/// The number of seconds in a minute as a Rational64.
pub(super) const SECONDS_PER_MINUTE: Rational64 = Rational64::new_raw(60, 1);
/// The number of seconds in an hour as a Rational64.
pub(super) const SECONDS_PER_HOUR: Rational64 = Rational64::new_raw(60 * 60, 1);

/// The number of performations in a foot of 35mm film.
pub(super) const PERFS_PER_FOOT_35: i64 = 64;

/// The number of performations in six inches of 16mm film.
/// 16mm film counts in an XX+YY form, as 35mm, but YY rolls over every 20 frames,
/// and there are 40 frames in a 16mm foot.
pub(super) const PERFS_PER_6INCHES_16: i64 = 20;

/// The number of ticks Adobe Premiere Pro breaks a second ratio.
pub(super) const PREMIERE_TICKS_PER_SECOND: Ratio<i128> = Ratio::<i128>::new_raw(254016000000, 1);

/// The number of seconds in a minute as an i64.
pub(super) const SECONDS_PER_MINUTE_I64: i64 = 60;
/// The number of seconds in an hour as an i64.
pub(super) const SECONDS_PER_HOUR_I64: i64 = SECONDS_PER_MINUTE_I64 * 60;

lazy_static! {
    /// TIMECODE_REGEX is a regex for parsing timecode values.
    pub(super) static ref TIMECODE_REGEX: Regex = regex::Regex::new(
        r"^(?P<negative>-)?((?P<section1>[0-9]+)[:|;])?((?P<section2>[0-9]+)[:|;])?((?P<section3>[0-9]+)[:|;])?(?P<frames>[0-9]+)$"
    ).unwrap();
}

lazy_static! {
    /// TIMECODE_REGEX is a regex for parsing timecode values.
    pub(super) static ref FEET_AND_FRAMES_REGEX: Regex = regex::Regex::new(
        r"^(?P<negative>-)?(?P<feet>[0-9]+)\+(?P<frames>[0-9]+)(\.P<perf>[0-9])?$",
    ).unwrap();
}

lazy_static! {
    /// RUNTIME_REGEX is a regex for parsing runtime values.
    pub(super) static ref RUNTIME_REGEX: Regex = regex::Regex::new(
        r"^(?P<negative>-)?((?P<section1>[0-9]+)[:|;])?((?P<section2>[0-9]+)[:|;])?(?P<seconds>[0-9]+(\.[0-9]+)?)$",
    ).unwrap();
}
