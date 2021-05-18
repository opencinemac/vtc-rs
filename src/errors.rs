#[allow(unused)]
// we need this here for the doc links, but clippy doesnt like that it isn't being used in code.
use crate::{Framerate, Timecode};

/// Returned from [Framerate::with_timebase] and [Framerate::with_playback] when there is an
/// error parsing a [Framerate].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FramerateParseError {
    /// Returned when a bad NTSC playback or timebase rate is given.
    Ntsc(String),
    /// Returned when a bad Drop-frame playback or timebase rate is given.
    DropFrame(String),
    /// Returned when a negative value is attempted to be converted to a Framerate.
    Negative(String),
    /// Returned when a value is not precise enough to be cast to a non-ntsc value, such as
    /// floating-point values.
    ///
    /// NTSC values have known denominators they must adhere to, and therefore can be coerced from
    /// imprecise values. No such coercion can be done for non-NTSC values.
    Imprecise(String),
    /// Returned when there is an error doing an internal type conversion to create a new Framerate,
    /// such as a u64 value overflowing a [num::Rational64].
    Conversion(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Returned from [Timecode::with_frames], [Timecode::with_seconds], and
/// [Timecode::with_premiere_ticks] when there is an error parsing a [Timecode].
pub enum TimecodeParseError {
    /// Returned when there is an error doing an internal type conversion to create a new Timecode,
    /// such as a u64 value overflowing a [num::Rational64].
    Conversion(String),
    /// Returned when a string does not match any known Timecode format.
    UnknownStrFormat(String),
    /// Returned when a drop-frame tc-string has a frames value that should have been dropped.
    /// ex: '00:01:00:01'.
    DropFrameValue(String),
}
