use crate::errors::FramerateParseError;
use crate::framerate_parse::FramerateSource;
use num::ToPrimitive;
use std::fmt;
use std::fmt::Formatter;

type ParseResult = Result<Framerate, FramerateParseError>;

/// NTSC is the type of NTSC standard a framerate adheres to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ntsc {
    /// None means this Framerate is not NTSC.
    None,
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
    /// println!("None: {}", vtc::Ntsc::None.is_ntsc());
    /// println!("NonDropFrame: {}", vtc::Ntsc::NonDropFrame.is_ntsc());
    /// println!("DropFrame: {}", vtc::Ntsc::DropFrame.is_ntsc());
    /// ```
    pub fn is_ntsc(self) -> bool {
        self != Self::None
    }
}

impl fmt::Display for Ntsc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ntsc_str = match self {
            Ntsc::None => "",
            Ntsc::NonDropFrame => "NTSC NDF",
            Ntsc::DropFrame => "NTSC DF",
        };
        write!(f, "{}", ntsc_str)
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
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::{Framerate, Ntsc};
    /// let rate = Framerate::new_with_playback("24000/1001", Ntsc::NonDropFrame).unwrap();
    /// println!("{}", rate.ntsc())
    /// ```
    pub fn ntsc(&self) -> Ntsc {
        self.ntsc
    }

    /// new_with_playback creates a new Framerate with a given real-world media playback value
    /// measured in frames-per-second.
    ///
    /// # Arguments
    ///
    /// * `rate` - A value that represents playback frames-per-second.
    ///
    /// * `ntsc` - The ntsc standard this value should be parsed as.
    ///
    /// # Examples
    ///
    /// we can generate any NTSC framerates from f32 or f64 values easily.
    ///
    /// ```rust
    /// use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback(23.98, Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// floats are automatically rounded to the nearest valid NTSC playback speed:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback(23.5, Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// strings can be parsed if they are a float or rational format:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback("23.98", Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback("24000/1001", Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// Non-valid NTSC playback rates will result in an error if we are parsing NTSC drop or
    /// non-drop values:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let err = Framerate::new_with_playback("24/1", Ntsc::NonDropFrame);
    /// println!("ERR: {:?}", err);
    /// ```
    ///
    /// This means that integers will always result in an error if ntsc != Ntsc::None:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let err = Framerate::new_with_playback(24, Ntsc::NonDropFrame);
    /// println!("ERR: {:?}", err);
    /// ```
    ///
    /// If we switch our NTSC settings to Ntsc::None, we can parse integers and integer strings, as
    /// well as other arbirary playback speed values:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback(24, Ntsc::None).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback("24", Ntsc::None).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_playback("3/1", Ntsc::None).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    pub fn new_with_playback<T: FramerateSource>(rate: T, ntsc: Ntsc) -> ParseResult {
        let rational = rate.to_playback(ntsc, false)?;
        let rate = Framerate {
            value: rational,
            ntsc,
        };
        Ok(rate)
    }

    /// new_with_timebase creates a new Framerate with a given timecode timebase playback value
    /// measured in frames-per-second. For NTSC framerates, the timebase will differ from the
    /// playback.
    ///
    /// # Arguments
    ///
    /// * `base` - A value that represents timebase frames-per-second.
    ///
    /// * `ntsc` - The ntsc standard this value should be parsed as.
    ///
    /// # Examples
    ///
    /// we can generate any NTSC framerates from any non 128-bit integer type easily:
    ///
    /// ```rust
    /// use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_timebase(24, Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// floats are automatically rounded to the nearest valid NTSC timebase speed:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_timebase(24.0, Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// strings can be parsed if they are an int, float or rational format:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_timebase("24", Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_timebase("24.0", Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_timebase("24/1", Ntsc::NonDropFrame).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
    ///
    /// Non-valid NTSC timebase will result in an error if we are parsing NTSC drop or non-drop
    /// values:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let err = Framerate::new_with_timebase("24000/1001", Ntsc::NonDropFrame);
    /// println!("ERR: {:?}", err);
    /// ```
    ///
    /// If we switch our NTSC settings, we can parse arbirary Framerate values:
    ///
    /// ```rust
    /// # use vtc::{Framerate, FramerateSource, Ntsc};
    /// let rate = Framerate::new_with_timebase("3/1", Ntsc::None).unwrap();
    /// println!("PLAYBACK: {}", rate.playback());
    /// println!("TIMEBASE: {}", rate.timebase());
    /// println!("NTSC    : {}", rate.ntsc());
    /// ```
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

        write!(f, "[{}", value_str)?;
        if self.ntsc.is_ntsc() {
            write!(f, " ")?;
        }
        write!(f, "{}]", self.ntsc)
    }
}

/// rates is a collection of common framerates seen in the wild as constants.
///
/// # Examples
///
/// ```rust
/// use vtc::rates;
/// println!("{}", rates::F23_98);
/// println!("{}", rates::F24);
/// println!("{}", rates::F29_97_DF);
/// ```
pub mod rates {
    use crate::Framerate;
    use crate::Ntsc;

    /// 23.98 NTSC.
    pub const F23_98: Framerate = Framerate {
        value: num::Rational64::new_raw(24000, 1001),
        ntsc: Ntsc::NonDropFrame,
    };

    /// 24 fps.
    pub const F24: Framerate = Framerate {
        value: num::Rational64::new_raw(24, 1),
        ntsc: Ntsc::None,
    };

    /// 29.97 NTSC Non-drop-frame.
    pub const F29_97_NDF: Framerate = Framerate {
        value: num::Rational64::new_raw(30000, 1001),
        ntsc: Ntsc::NonDropFrame,
    };

    /// 29.97 NTSC Drop-frame.
    pub const F29_97_DF: Framerate = Framerate {
        value: num::Rational64::new_raw(30000, 1001),
        ntsc: Ntsc::DropFrame,
    };

    /// 30 fps.
    pub const F30: Framerate = Framerate {
        value: num::Rational64::new_raw(30, 1),
        ntsc: Ntsc::None,
    };

    /// 47.95 NTSC.
    pub const F47_95: Framerate = Framerate {
        value: num::Rational64::new_raw(48000, 1001),
        ntsc: Ntsc::NonDropFrame,
    };

    /// 48 fps.
    pub const F48: Framerate = Framerate {
        value: num::Rational64::new_raw(48, 1),
        ntsc: Ntsc::None,
    };

    /// 59.94 NTSC Non-drop-frame.
    pub const F59_94_NDF: Framerate = Framerate {
        value: num::Rational64::new_raw(60000, 1001),
        ntsc: Ntsc::NonDropFrame,
    };

    /// 59.94 NTSC Drop-frame.
    pub const F59_94_DF: Framerate = Framerate {
        value: num::Rational64::new_raw(60000, 1001),
        ntsc: Ntsc::DropFrame,
    };

    /// 60 fps.
    pub const F60: Framerate = Framerate {
        value: num::Rational64::new_raw(60, 1),
        ntsc: Ntsc::None,
    };
}
