/// FramerateParseErr is returned when there is an error parsing a framerate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FramerateParseError {
    /// Ntsc is returned when a bad NTSC playback or timebase rate is given.
    Ntsc(String),
    /// DropFrame is returned when a bad Drop-frame playback or timebase rate is given.
    DropFrame(String),
    /// Negative is returned when a negative value is attempted to be converted to a Framerate.
    Negative(String),
    /// Imprecise is returned when a value is not precise enough to be cast to a non-ntsc value,
    /// such as floating-point values.
    ///
    /// NTSC values have known denominators they must adhere to, and therefore can be coerced from
    /// imprecise values. No such coercion can be done for non-NTSC values.
    Imprecise(String),
    /// Conversion is returned when there is an error doing an internal type conversion to create a
    /// new Framerate, such as a u64 value overflowing a num::Rational64.
    Conversion(String),
}
