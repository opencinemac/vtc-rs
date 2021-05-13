#[allow(unused)]
// we need this here for the doc links, but clippy doesnt like that it isn't being used in code.
use crate::Framerate;

/// Returned from [Framerate::new_with_timebase] and [Framerate::new_with_playback] when there is an
/// error parsing a framerate.
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
