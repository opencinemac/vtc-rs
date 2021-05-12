use num::{FromPrimitive, ToPrimitive};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

/// NTSC is the type of NTSC standard a framerate adheres to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ntsc {
    /// False means this Framerate is not NTSC.
    False(),
    /// NonDrop means this Framerate is non-drop NTSC (no frame numbers are dropped to sync timecode
    /// with real-world time - results in Timecode that drifts from true time).
    NonDropFrame(),
    /// DropFrame means this framerate is drop-frame NTSC (frames numbers are dropped periodically
    /// to keep timecode in sync with real-world time).
    DropFrame(),
}

impl Ntsc {
    /// check returns whether this is any NTSC format (drop or non-drop).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::Ntsc;
    /// println!("False: {}", vtc::Ntsc::False().check());
    /// println!("NonDropFrame: {}", vtc::Ntsc::NonDropFrame().check());
    /// println!("DropFrame: {}", vtc::Ntsc::DropFrame().check());
    /// ```
    pub fn check(self) -> bool {
        self != Self::False()
    }

    /// is_non_drop returns true if this is an NTSC non-drop-frame specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::Ntsc;
    /// println!("False: {}", vtc::Ntsc::False().is_non_dropframe());
    /// println!("NonDropFrame: {}", vtc::Ntsc::NonDropFrame().is_non_dropframe());
    /// println!("DropFrame: {}", vtc::Ntsc::DropFrame().is_non_dropframe());
    /// ```
    pub fn is_non_dropframe(self) -> bool {
        self == Self::NonDropFrame()
    }

    /// is_dropframe returns true if this is an NTSC drop-frame specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::Ntsc;
    /// println!("False: {}", vtc::Ntsc::False().is_dropframe());
    /// println!("NonDropFrame: {}", vtc::Ntsc::NonDropFrame().is_dropframe());
    /// println!("DropFrame: {}", vtc::Ntsc::DropFrame().is_dropframe());
    /// ```
    pub fn is_dropframe(self) -> bool {
        self == Self::DropFrame()
    }
}

/// Framerate is the rate at which a video file frames are played back.
///
/// Framerate is measured in frames-per-second (24/1 = 24 frames-per-second).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Framerate {
    value: num::Rational64,
    ntsc: Ntsc,
}

impl Framerate {
    /// playback is the rational representation of the real-world playback speed as a fraction
    /// in frames-per-second.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::{Framerate, Ntsc};
    /// let rate = Framerate::from_str_timebase("24/1", Ntsc::NonDropFrame()).unwrap();
    /// println!("{}", rate.playback())
    /// ```
    pub fn playback(&self) -> num::Rational64 {
        self.value
    }

    /// timebase is the rational representation of the timecode timebase speed as a fraction in
    /// frames-per-second.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::{Framerate, Ntsc};
    /// let rate = Framerate::from_str_playback("24000/1001", Ntsc::NonDropFrame()).unwrap();
    /// println!("{}", rate.timebase())
    /// ```
    pub fn timebase(&self) -> num::Rational64 {
        // If this is an NTSC timebase, we need to round it to the nearest whole number.
        if self.ntsc.check() {
            return self.value.round();
        }
        self.value
    }

    /// ntsc is whether this is an NTSC-style time base (aka 23.98, 24000/1001, etc). It returns
    /// an enum detailing if it is not NTSC or what type of NTSC flavor it is.
    pub fn ntsc(&self) -> Ntsc {
        self.ntsc
    }

    /// from_rational is the private method we are going to use for parsing from both timebases and
    /// framerates.
    fn from_rational(value: num::Rational64, ntsc: Ntsc, is_base: bool) -> Result<Self, String> {
        validate_ntsc_value(value, ntsc, is_base)?;

        let mut new_value = value;
        // if this is a timebase and it's NTSC, we need to convert it to a framerate.
        if is_base && ntsc.check() {
            new_value = new_value.round() * 1000 / 1001
        }

        let new = Framerate {
            value: new_value,
            ntsc,
        };

        Ok(new)
    }

    /// from_rational_base creates a new framerate from a fraction representing the timebase of the
    /// framerate.
    pub fn from_rational_timebase(base: num::Rational64, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_rational(base, ntsc, true)
    }

    /// from_rational_rate creates a new framerate from a fraction representing the timebase of the
    /// framerate.
    pub fn from_rational_playback(rate: num::Rational64, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_rational(rate, ntsc, false)
    }

    /// from_i64 is the private method we are going to use for parsing from both timebases and
    /// framerates from 64-bit integers.
    fn from_i64(value: i64, ntsc: Ntsc, is_base: bool) -> Result<Self, String> {
        let rational = num::Rational64::from_integer(value);
        Self::from_rational(rational, ntsc, is_base)
    }

    /// from_i64_timebase creates a new framerate from a 64-bit integer representing the timecode
    /// timebase speed in frames-per-second.
    pub fn from_i64_timebase(base: i64, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_i64(base, ntsc, true)
    }

    /// from_i64_timebase creates a new framerate from a 64-bit integer representing the playback
    /// speed in frames-per-second. This method does not expose ntsc or drop-frame flags, as ntsc
    /// playback framerate cannot be whole-frame.
    pub fn from_i64_playback(rate: i64) -> Result<Self, String> {
        Self::from_i64(rate, Ntsc::False(), false)
    }

    /// from_f64 creates a new framerate from a 64-bit float. If ntsc=true, the value will be
    /// coerced to the nearest ntsc value (denominator of 1001).
    fn from_f64(value: f64, ntsc: Ntsc, is_base: bool) -> Result<Self, String> {
        if !ntsc.check() {
            return Err("float values cannot be parsed if NTSC is not true".to_string());
        }
        let mut rational = match num::Rational64::from_f64(value) {
            None => return Err("could not parse rational from f64".to_string()),
            Some(rational) => rational,
        };

        // If this is an NTSC playback speed, coerce it to the nearest correct ntsc value.
        if !is_base && ntsc.check() {
            rational = rational.round() * 1000 / 1001
        }

        Self::from_rational(rational, ntsc, is_base)
    }

    /// from_f64_timebase creates a new framerate from a 64-bit float representing the timecode
    /// timebase speed in frames-per-second.
    pub fn from_f64_timebase(base: f64, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_f64(base, ntsc, false)
    }

    /// from_f64_timebase creates a new framerate from a 64-bit float representing the playback
    /// speed in frames-per-second.
    pub fn from_f64_playback(rate: f64, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_f64(rate, ntsc, false)
    }

    /// from_str is the private method we are going to use for parsing from both timebases and
    /// framerates from strings.
    fn from_str(value: &str, ntsc: Ntsc, is_base: bool) -> Result<Self, String> {
        let mut result: Option<Result<Self, String>> = None;

        if let Ok(parsed) = num::Rational64::from_str(value) {
            return Self::from_rational(parsed, ntsc, is_base);
        }

        if let Ok(parsed) = i64::from_str(value) {
            return Self::from_i64(parsed, ntsc, is_base);
        }

        if let Ok(parsed) = f64::from_str(value) {
            result = Some(Self::from_f64(parsed, ntsc, is_base))
        }

        match result {
            None => Err(format!(
                "could not parse '{0}' as rational, int, or float for framerate.",
                value
            )),
            Some(result) => result,
        }
    }

    /// from_str_timebase creates a new framerate from a &str representing the timecode timebase
    /// speed in frames-per-second.
    ///
    /// strings may be formatted as:
    ///     - a fraction '24/1'
    ///     - a float    '24.0'
    ///     - an integer '24'
    pub fn from_str_timebase(base: &str, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_str(base, ntsc, true)
    }

    /// from_str_timebase creates a new framerate from a &str representing the timecode timebase
    /// speed in frames-per-second.
    ///
    /// strings may be formatted as:
    ///     - a fraction '24000/1001'
    ///     - a float    '23.98'
    ///     - an integer '24'
    pub fn from_str_playback(rate: &str, ntsc: Ntsc) -> Result<Self, String> {
        Self::from_str(rate, ntsc, false)
    }
}

impl fmt::Display for Framerate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value_str = format!("{:.2}", self.value.to_f64().unwrap());
        let mut value_str = value_str.trim_end_matches('0');
        value_str = value_str.trim_end_matches('.');

        let ntsc_str = match self.ntsc {
            Ntsc::False() => "",
            Ntsc::NonDropFrame() => " NTSC",
            Ntsc::DropFrame() => " NTSC DF",
        };
        write!(f, "[{}{}]", value_str, ntsc_str)
    }
}

const DROP_DIVISOR_PLAYBACK: num::Rational64 = num::Rational64::new_raw(30000, 1001);
const DROP_DIVISOR_TIMEBASE: num::Rational64 = num::Rational64::new_raw(30, 1);

/// validate_dropframe_ntsc validates that our ntsc and drop-frame settings are correct.
fn validate_ntsc_value(value: num::Rational64, ntsc: Ntsc, is_base: bool) -> Result<(), String> {
    // If this is not an NTSC value, return immediately.
    if !ntsc.check() {
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
    if !ntsc.is_dropframe() {
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
            "dropframe may only be true if {0} is divisible by {1} (multiple of 29.97)",
            rate_type, drop_divisor,
        );
        return Err(err_message);
    }

    // If we get to here, everything is a-ok!
    Ok(())
}
