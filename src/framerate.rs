use crate::errors::ParseErr;
use crate::framerate_parse::FramerateSource;
use num::ToPrimitive;
use std::fmt;
use std::fmt::Formatter;

type ParseResult = Result<Framerate, ParseErr>;

/// NTSC is the type of NTSC standard a framerate adheres to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ntsc {
    /// False means this Framerate is not NTSC.
    False,
    /// NonDrop means this Framerate is non-drop NTSC (no frame numbers are dropped to sync timecode
    /// with real-world time - results in Timecode that drifts from true time).
    NonDropFrame,
    /// DropFrame means this framerate is drop-frame NTSC (frames numbers are dropped periodically
    /// to keep timecode in sync with real-world time).
    DropFrame,
}

impl Ntsc {
    /// is_ntsc returns whether this is any NTSC format (drop or non-drop).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::Ntsc;
    /// println!("False: {}", vtc::Ntsc::False.is_ntsc());
    /// println!("NonDropFrame: {}", vtc::Ntsc::NonDropFrame.is_ntsc());
    /// println!("DropFrame: {}", vtc::Ntsc::DropFrame.is_ntsc());
    /// ```
    pub fn is_ntsc(self) -> bool {
        self != Self::False
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
    /// let rate = Framerate::new_with_timebase("24/1", Ntsc::NonDropFrame).unwrap();
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
    /// let rate = Framerate::new_with_playback("24000/1001", Ntsc::NonDropFrame).unwrap();
    /// println!("{}", rate.timebase())
    /// ```
    pub fn timebase(&self) -> num::Rational64 {
        // If this is an NTSC timebase, we need to round it to the nearest whole number.
        if self.ntsc.is_ntsc() {
            return self.value.round();
        }
        self.value
    }

    /// ntsc is whether this is an NTSC-style time base (aka 23.98, 24000/1001, etc). It returns
    /// an enum detailing if it is not NTSC or what type of NTSC flavor it is.
    pub fn ntsc(&self) -> Ntsc {
        self.ntsc
    }

    pub fn new_with_playback<T: FramerateSource>(rate: T, ntsc: Ntsc) -> ParseResult {
        let rational = rate.to_playback(ntsc, false)?;
        let rate = Framerate {
            value: rational,
            ntsc,
        };
        Ok(rate)
    }

    pub fn new_with_timebase<T: FramerateSource>(base: T, ntsc: Ntsc) -> ParseResult {
        let rational = base.to_playback(ntsc, true)?;
        let rate = Framerate {
            value: rational,
            ntsc,
        };
        Ok(rate)
    }
}

impl fmt::Display for Framerate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value_str = format!("{:.2}", self.value.to_f64().unwrap());
        let mut value_str = value_str.trim_end_matches('0');
        value_str = value_str.trim_end_matches('.');

        let ntsc_str = match self.ntsc {
            Ntsc::False => "",
            Ntsc::NonDropFrame => " NTSC",
            Ntsc::DropFrame => " NTSC DF",
        };
        write!(f, "[{}{}]", value_str, ntsc_str)
    }
}
