use crate::errors::FramerateParseError;
use crate::framerate_parse::FramerateSource;
use num::ToPrimitive;
use std::fmt;
use std::fmt::Formatter;

#[allow(unused)] // for docs links
use num::Rational64;

/// The type of NTSC standard a [Framerate] adheres to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ntsc {
    /// This [Framerate] is not NTSC.
    None,
    /// This [Framerate] is non-drop NTSC (no frame numbers are dropped to sync timecode with
    /// real-world time - results in Timecode that drifts from true time).
    NonDropFrame,
    /// This [Framerate] is drop-frame NTSC (frames numbers are dropped periodically to keep
    /// timecode in sync with real-world time).
    DropFrame,
}

impl Ntsc {
    /// Returns whether this is any NTSC format (drop or non-drop).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::Ntsc;
    /// assert!(!vtc::Ntsc::None.is_ntsc());
    /// assert!(vtc::Ntsc::NonDropFrame.is_ntsc());
    /// assert!(vtc::Ntsc::DropFrame.is_ntsc());
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

/// The [Result] type returned by [Framerate::with_playback] and [Framerate::with_timebase].
pub type FramerateParseResult = Result<Framerate, FramerateParseError>;

/// The rate at which a video file frames are played back.
///
/// Framerate is measured in frames-per-second (24/1 = 24 frames-per-second).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Framerate {
    value: num::Rational64,
    ntsc: Ntsc,
}

impl Framerate {
    /// The rational representation of the real-world playback speed as a fraction in
    /// frames-per-second.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::{Framerate, Ntsc};
    /// use num::Rational64;
    /// let rate = Framerate::with_timebase("24/1", Ntsc::NonDropFrame).unwrap();
    /// assert_eq!(Rational64::new(24000, 1001), rate.playback())
    /// ```
    pub fn playback(&self) -> num::Rational64 {
        self.value
    }

    /// The rational representation of the timecode timebase speed as a fraction in
    /// frames-per-second.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::{Framerate, Ntsc};
    /// use num::Rational64;
    /// let rate = Framerate::with_playback("24000/1001", Ntsc::NonDropFrame).unwrap();
    /// assert_eq!(Rational64::new(24, 1), rate.timebase())
    /// ```
    pub fn timebase(&self) -> num::Rational64 {
        // If this is an NTSC timebase, we need to round it to the nearest whole number.
        if self.ntsc.is_ntsc() {
            return self.value.round();
        }
        self.value
    }

    /// Whether this is an NTSC-style time base (aka 23.98, 24000/1001, etc). Returns an enum
    /// detailing if it is not NTSC or what type of NTSC flavor it is.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vtc::{Framerate, Ntsc};
    /// let rate = Framerate::with_playback("24000/1001", Ntsc::NonDropFrame).unwrap();
    /// assert_eq!(Ntsc::NonDropFrame, rate.ntsc())
    /// ```
    pub fn ntsc(&self) -> Ntsc {
        self.ntsc
    }

    /// drop_frames returns the number of frames to skip on non-10th minutes in drop
    /// frame timecodes. This method will return None on non-dop Framerates
    ///
    /// Algorithm adapted from: https://www.davidheidelberger.com/2010/06/10/drop-frame-timecode/
    pub(crate) fn drop_frames_per_minute(&self) -> Option<i64> {
        if self.ntsc != Ntsc::DropFrame {
            return None;
        }

        let drop_frames = self.timebase().round().to_integer() as f64 * 0.066666;
        Some(drop_frames.round() as i64)
    }

    /**
    Creates a new [Framerate] with a given real-world media playback value measured in
    frames-per-second.

    # Arguments

    * `rate` - A value that represents playback frames-per-second.

    * `ntsc` - The ntsc standard this value should be parsed as.

    # Examples

    We can generate any NTSC framerates from [f32] or [f64] values easily.

    ```rust
    use vtc::{Framerate, FramerateSource, Ntsc};
    use num::Rational64;
    let rate = Framerate::with_playback(23.98, Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    Floats are automatically rounded to the nearest valid NTSC playback speed:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_playback(23.5, Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    [&str] and [String] can be parsed if they are a float or rational format:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_playback("23.98", Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_playback("24000/1001", Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    Non-valid NTSC playback rates will result in an error if we are parsing NTSC drop or
    non-drop values:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc, FramerateParseError};
    let err = Framerate::with_playback("24/1", Ntsc::NonDropFrame);
    assert_eq!(FramerateParseError::Ntsc("ntsc framerates must be n/1001".to_string()), err.err().unwrap());
    ```

    This means that integers will always result in an error if ntsc != [Ntsc::None]:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    let err = Framerate::with_playback(24, Ntsc::NonDropFrame);
    assert!(err.is_err());
    ```

    If we switch our NTSC settings to [Ntsc::None], we can parse integers and integer strings,
    as well as other arbirary playback speed values:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_playback(24, Ntsc::None).unwrap();
    assert_eq!(Rational64::new(24, 1), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::None, rate.ntsc());
    ```

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_playback("24", Ntsc::None).unwrap();
    assert_eq!(Rational64::new(24, 1), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::None, rate.ntsc());
    ```

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_playback("3/1", Ntsc::None).unwrap();
    assert_eq!(Rational64::new(3, 1), rate.playback());
    assert_eq!(Rational64::new(3, 1), rate.timebase());
    assert_eq!(Ntsc::None, rate.ntsc());
    ```

    If we try to parse a non-drop-frame NTSC value with the wrong timbebase we will get an error:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc, FramerateParseError};
    let err = Framerate::with_playback(23.98, Ntsc::DropFrame);
    assert_eq!(FramerateParseError::DropFrame("dropframe must have playback divisible by 30000/1001 (multiple of 29.97)".to_string()), err.err().unwrap());
    ```

    For more information on why drop-frame timebases must be a multiple of 30000/1001, see
    [this blogpost](https://www.davidheidelberger.com/2010/06/10/drop-frame-timecode/).

    # note

    Using a float with [Ntsc::None] will result in an error. Floats are not precise, and without
    the ntsc flag, vtc cannot know exactly what framerate you want. A [Rational64] value must
    be used.
    */
    pub fn with_playback<T: FramerateSource>(rate: T, ntsc: Ntsc) -> FramerateParseResult {
        let rational = rate.to_playback(ntsc, false)?;
        let rate = Framerate {
            value: rational,
            ntsc,
        };
        Ok(rate)
    }

    /**
    Creates a new [Framerate] with a given timecode timebase playback value measured in
    frames-per-second. For NTSC framerates, the timebase will differ from the playback.

    # Arguments

    * `base` - A value that represents timebase frames-per-second.

    * `ntsc` - The ntsc standard this value should be parsed as.

    # Examples

    We can generate any NTSC framerates from any non 128-bit integer type easily:

    ```rust
    use vtc::{Framerate, FramerateSource, Ntsc};
    use num::Rational64;
    let rate = Framerate::with_timebase(24, Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    Floats are automatically rounded to the nearest valid NTSC timebase speed:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_timebase(24.0, Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    [&str] and [String] can be parsed if they are an int, float or rational format:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_timebase("24", Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_timebase("24.0", Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_timebase("24/1", Ntsc::NonDropFrame).unwrap();
    assert_eq!(Rational64::new(24000, 1001), rate.playback());
    assert_eq!(Rational64::new(24, 1), rate.timebase());
    assert_eq!(Ntsc::NonDropFrame, rate.ntsc());
    ```

    Non-valid NTSC timebase will result in an error if we are parsing NTSC drop or non-drop
    values:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc, FramerateParseError};
    let err = Framerate::with_timebase("24000/1001", Ntsc::NonDropFrame);
    assert_eq!(
        FramerateParseError::Ntsc("ntsc timebases must be whole numbers".to_string()),
        err.err().unwrap(),
    );
    ```

    If we switch our NTSC settings, we can parse arbirary Framerate values:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc};
    # use num::Rational64;
    let rate = Framerate::with_timebase("3/1", Ntsc::None).unwrap();
    assert_eq!(Rational64::new(3, 1), rate.playback());
    assert_eq!(Rational64::new(3, 1), rate.timebase());
    assert_eq!(Ntsc::None, rate.ntsc());
    ```

    If we try to parse a drop-frame value with the wrong timbebase we will get an error:

    ```rust
    # use vtc::{Framerate, FramerateSource, Ntsc, FramerateParseError};
    let err = Framerate::with_timebase("24", Ntsc::DropFrame);
    assert_eq!(
        FramerateParseError::DropFrame("dropframe must have timebase divisible by 30 (multiple of 29.97)".to_string()),
        err.err().unwrap(),
    );
    ```

    For more information on why drop-frame timebases must be a multiple of 30, see
    [this blogpost](https://www.davidheidelberger.com/2010/06/10/drop-frame-timecode/).
    */
    pub fn with_timebase<T: FramerateSource>(base: T, ntsc: Ntsc) -> FramerateParseResult {
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

/**
A collection of common framerates seen in the wild as constants.

# Examples

```rust
use vtc::{rates, Framerate, Ntsc};
assert_eq!(rates::F23_98, Framerate::with_playback(23.98, Ntsc::NonDropFrame).unwrap());
assert_eq!(rates::F24, Framerate::with_playback(24, Ntsc::None).unwrap());
assert_eq!(rates::F29_97_NDF, Framerate::with_playback(29.97, Ntsc::NonDropFrame).unwrap());
assert_eq!(rates::F29_97_DF, Framerate::with_playback(29.97, Ntsc::DropFrame).unwrap());
```
*/
pub mod rates {
    use crate::Framerate;
    use crate::Ntsc;

    /// 23.98 NTSC Non-drop-frame.
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
