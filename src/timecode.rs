use num::integer::{div_rem, lcm};
use num::rational::Ratio;
use num::{abs, FromPrimitive, Rational64, Signed, ToPrimitive, Zero};

use crate::{
    consts::{
        PERFS_PER_6INCHES_16, PERFS_PER_FOOT_35, PREMIERE_TICKS_PER_SECOND, SECONDS_PER_HOUR,
        SECONDS_PER_MINUTE,
    },
    source_ppro_ticks::PremiereTicksSource,
    timecode_parse::round_seconds_to_frame,
    Framerate, FramesSource, Ntsc, SecondsSource, TimecodeParseError,
};
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub};
use std::{cmp::Ordering, ops::AddAssign};
use std::{
    fmt::{Display, Formatter},
    ops::SubAssign,
};

/**
Holds the individual sections of a timecode for formatting / manipulation.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimecodeSections {
    /// Whether the timecode is a negative value.
    pub negative: bool,
    /// Hours place value.
    pub hours: i64,
    /// Minutes place value.
    pub minutes: i64,
    /// Seconds place value.
    pub seconds: i64,
    /// Frames value.
    pub frames: i64,
}

/** Feet and Frames Representations

[[FF35mm4perf]]

35mm 4-perf film (16 frames per foot). ex: '5400+13'.

# What it is

On physical film, each foot contains a certain number of frames. For 35mm, 4-perf film (the most
common type on Hollywood movies), this number is 16 frames per foot. Feet-And-Frames was often
used in place of Keycode to quickly reference a frame in the edit.

# Where you see it

For the most part, feet + frames has died out as a reference, because digital media is not
measured in feet. The most common place it is still used is Studio Sound Departments. Many Sound
Mixers and Designers intuitively think in feet + frames, and it is often burned into the
reference picture for them.

- Telecine.
- Sound turnover reference picture.
- Sound turnover change lists.

[[FF35mm3perf]]

35mm film with 3-perf per-frame pulldown (64 perforations per foot). ex: '12+01.1'.

# What it is

3-perf footages are represented as a string with three terms, a number of feet, a
number of frames from the beginning of the foot, and an final framing term preceded by
a period, indicating how many perfs were cut off the first frame of this foot.

# Where you see it

Avid cutlists and pull lists on Avid 3 perf projects.

[[FF35mm2perf]]

# What it is

35mm 2-perf film records 32 frames in a foot of film, instead of the usual 16.
This creates a negative image with a wide aspect ratio using standard spherical
lenses and consumes half the footage per minute running time as standard 35mm,
while having a grain profile somewhat better than 16mm while not as good as
standard 35mm.

# Where you see it

35mm 2-perf formats are uncommon though still find occasional use, the process is
usually marketed as "Techniscope", the original trademark for Technicolor Italia's
2-perf format. It was historically very common in the Italian film industry prior
to digital filmmaking, and is used on some contemporary films to obtain a film look
while keeping stock and processing costs down.

[[FF16mm]]

# What it is

On 16mm film, there are forty frames of film in each foot, one perforation
per frame. However, 16mm film is edge coded every six inches, with twenty
frames per code, so the footage "1+19" is succeeded by "2+0".

# Where you see it

16mm telecines, 16mm edge codes.

*/
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FilmFormat {
    /// 35mm, 4-perf footage
    FF35mm4perf,
    /// 35mm, 3-perf footage
    FF35mm3perf,
    /// 35mm, 2-perf footage
    FF35mm2perf,
    /// 16mm footage
    FF16mm,
}

impl FilmFormat {
    /// Utility function mapping self to number
    /// of perfs per (logical) foot in this case.
    pub fn perfs_per_foot(&self) -> i64 {
        match self {
            FilmFormat::FF16mm => PERFS_PER_6INCHES_16,
            FilmFormat::FF35mm4perf | FilmFormat::FF35mm3perf | FilmFormat::FF35mm2perf => {
                PERFS_PER_FOOT_35
            }
        }
    }

    /// Fewest number of perfs required to complete an integral
    /// number of feet and integral number of frames in this
    /// format. (This should simply be the lcm of the perfs-per-frame
    /// and the perfs-per-foot.)
    pub fn footage_modulus_perf_count(&self) -> i64 {
        lcm(self.perfs_per_frame(), self.perfs_per_foot())
    }

    /// Number of frames in footage_perf_modulus.
    pub fn footage_modulus_frame_count(&self) -> i64 {
        self.footage_modulus_perf_count() / self.perfs_per_frame()
    }

    /// Number of feet in footage_perf_modulus.
    pub fn footage_modulus_footage_count(&self) -> i64 {
        self.footage_modulus_perf_count() / self.perfs_per_foot()
    }

    /// Utility function mapping self to number of
    /// perfs per frame in this case.
    pub fn perfs_per_frame(&self) -> i64 {
        match self {
            FilmFormat::FF16mm => 1,
            FilmFormat::FF35mm4perf => 4,
            FilmFormat::FF35mm3perf => 3,
            FilmFormat::FF35mm2perf => 2,
        }
    }

    /// Utility function indicating if the format requires
    /// a perf field in footage string representations.
    pub fn allows_perf_field(&self) -> bool {
        self.perfs_per_foot() % self.perfs_per_frame() != 0
    }
}

/// A struct that bundles a str together with a format, to hint parsing.
///
/// This struct, which implelements [FrameSource], allows you to include
/// a format hint to the footage parser, which will override its default
/// inference.
#[derive(Debug)]
pub struct FeetFramesStr<'a> {
    pub(crate) input: &'a str,
    pub(crate) format: FilmFormat,
}

impl<'a> FeetFramesStr<'a> {
    /// Create a [FeetFramesStr] object from a string and a [FilmFormat].
    pub fn new(input: &'a str, format: FilmFormat) -> Self {
        FeetFramesStr { input, format }
    }
}

/// The [Result] type returned by [Timecode::with_seconds], [Timecode::with_frames], and
/// [Timecode::with_premiere_ticks].
pub type TimecodeParseResult = Result<Timecode, TimecodeParseError>;

/**
[Timecode] represents the frame at a particular time in a video.

New [Timecode] values are created with the [Timecode::with_seconds], [Timecode::with_frames],
and [Timecode::with_premiere_ticks] methods.

[Timecode] is a [Copy] value.

# Examples

For timecode attribute examples, see the individual methods of the [Timecode] type, such as
[Timecode::timecode]. For examples of how to construct a new timecode, see the examples on the
contructor methods like [Timecode::with_frames].

There are a number of opeations we can apply to [Timecode] values.

## Compare Timecodes

```rust
use vtc::{Timecode, rates};
let tc1 = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
let tc2 = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
let tc3 = Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap();
let tc4 = Timecode::with_frames("01:30:00:00", rates::F23_98).unwrap();

assert!(tc1 == tc2);
assert!(tc1 > tc3);
assert!(tc1 < tc4);
```

## Sort_Timecodes

```rust
# use vtc::{Timecode, rates};
let tc1 = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
let tc2 = Timecode::with_frames("01:30:00:00", rates::F23_98).unwrap();
let tc3 = Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap();

let mut timecodes = vec![tc1, tc2, tc3];

timecodes.sort();

assert_eq!(timecodes[0], tc3);
assert_eq!(timecodes[1], tc1);
assert_eq!(timecodes[2], tc2);
```

## Add Timecodes

```rust
# use vtc::{Timecode, rates};
let tc1 = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
let tc2 = Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap();

let mut result = tc1 + tc2;

assert_eq!("01:30:00:00", result.timecode());

result += tc1;

assert_eq!("02:30:00:00", result.timecode());
```

## Subtract Timecodes

```rust
# use vtc::{Timecode, rates};
let tc1 = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
let tc2 = Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap();

let mut result = tc1 - tc2;

assert_eq!("00:30:00:00", result.timecode());

result -= tc1;

assert_eq!("-00:30:00:00", result.timecode());
```

## Multiply Timecodes

```rust
# use vtc::{Timecode, rates};
let tc1 = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();

let mut result = tc1 * 1.5;

assert_eq!("01:30:00:00", result.timecode());

result *= 2;

assert_eq!("03:00:00:00", result.timecode());
```

## Divide Timecodes

Dividing always acts as if floor devision had been done on the frame count of the [Timecode].

```rust
# use vtc::{Timecode, rates};
let tc1 = Timecode::with_frames("01:00:00:01", rates::F23_98).unwrap();

let result = tc1 / 1.5;

assert_eq!("00:40:00:00", result.timecode());
```

This allows divisions and remainders to give correct, complementary values:

```rust
# use vtc::{Timecode, rates};
# let tc1 = Timecode::with_frames("01:00:00:01", rates::F23_98).unwrap();
let result = tc1 % 1.5;

assert_eq!("00:00:00:01", result.timecode());
```

[DivAssign] and [RemAssign] are also implemented for [Timecode]:

```rust
# use vtc::{Timecode, rates};
let mut tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();

tc /= 2;

assert_eq!("00:30:00:00", tc.timecode());

tc %= 1.65;
assert_eq!("00:00:00:01", tc.timecode())
```

*/
#[derive(Clone, Copy, Debug)]
pub struct Timecode {
    seconds: Rational64,
    rate: Framerate,
}

impl Timecode {
    /// Returns the Framerate of the timecode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    /// assert_eq!(rates::F23_98, tc.rate())
    /// ```
    pub fn rate(&self) -> Framerate {
        self.rate
    }

    /**
    Returns the rational representation of the real-world seconds that would have elapsed
    between 00:00:00:00 and this timecode.

    # What it is

    The number of real-world seconds that have elapsed between 00:00:00:00 and the timecode value.
    With NTSC timecode, the timecode drifts from the real-world elapsed time.

    # Where you see it

    - Anywhere real-world time needs to be calculated.
    - In code that needs to do lossless calculations of playback time and not rely on frame count,
      like adding two timecodes together with different framerates.

    # Examples

    ```rust
    # use vtc::{Timecode, rates};
    use num::Rational64;
    let tc = Timecode::with_seconds(3600.0, rates::F24).unwrap();
    assert_eq!(Rational64::new(3600, 1), tc.seconds())
    ```
    */
    pub fn seconds(&self) -> Rational64 {
        self.seconds
    }

    /// The individual sections of a timecode string as i64 values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates, TimecodeSections};
    /// let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    /// let expected = TimecodeSections{
    ///     negative: false,
    ///     hours: 1,
    ///     minutes: 0,
    ///     seconds: 0,
    ///     frames: 0,
    /// };
    /// assert_eq!(expected, tc.sections())
    /// ```
    pub fn sections(&self) -> TimecodeSections {
        // We use the absolute frame count here so floor behaves as expected regardless of whether
        // this value is negative.
        let mut frames_int = abs(self.frames());
        let timebase = self.rate.timebase();

        if self.rate.ntsc() == Ntsc::DropFrame {
            // Convert the frame number to an adjusted one for drop-frame display values.
            frames_int += drop_frame_adjustment(frames_int, self.rate);
        }

        let mut frames = Rational64::from_integer(frames_int);

        let frames_per_minute = timebase * SECONDS_PER_MINUTE;
        let frames_per_hour = timebase * SECONDS_PER_HOUR;

        let hours = (frames / frames_per_hour).floor();
        frames %= frames_per_hour;

        let minutes = (frames / frames_per_minute).floor();
        frames %= frames_per_minute;

        let seconds = (frames / timebase).floor();
        frames = (frames % timebase).round();

        TimecodeSections {
            negative: self.seconds.is_negative(),
            hours: hours.to_integer(),
            minutes: minutes.to_integer(),
            seconds: seconds.to_integer(),
            frames: frames.to_integer(),
        }
    }

    /**
    Returns the the formatted SMPTE timecode: (ex: 01:00:00:00).

    # What it is

    Timecode is used as a human-readable way to represent the id of a given frame. It is formatted
    to give a rough sense of where to find a frame: {HOURS}:{MINUTES}:{SECONDS}:{FRAME}. For more on
    timecode, see Frame.io's
    [excellent post](https://blog.frame.io/2017/07/17/timecode-and-frame-rates/) on the subject.

    # Where you see it

    Timecode is ubiquitous in video editing, a small sample of places you might see timecode:

    - Source and Playback monitors in your favorite NLE.
    - Burned into the footage for dailies.
    - Cut lists like an EDL.

    # Examples

    ```rust
    # use vtc::{Timecode, rates};
    let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    assert_eq!("01:00:00:00", tc.timecode())
    ```
    */
    pub fn timecode(&self) -> String {
        let sections = self.sections();

        let sign = if self.seconds.is_negative() { "-" } else { "" };

        let frame_sep = if self.rate.ntsc() == Ntsc::DropFrame {
            ";"
        } else {
            ":"
        };

        format!(
            "{}{:02}:{:02}:{:02}{}{:02}",
            sign, sections.hours, sections.minutes, sections.seconds, frame_sep, sections.frames,
        )
    }

    /**
    Returns the number of frames that would have elapsed between 00:00:00:00 and this timecode.

    # What it is

    Frame number / frames count is the number of a frame if the timecode started at 00:00:00:00 and
    had been running until the current value. A timecode of '00:00:00:10' has a frame number of 10.
    A timecode of '01:00:00:00' has a frame number of 86400.

    # Where you see it

    - Frame-sequence files: 'my_vfx_shot.0086400.exr'
    - FCP7XML cut lists:

        ```xml
        <timecode>
            <rate>
                <timebase>24</timebase>
                <ntsc>TRUE</ntsc>
            </rate>
            <string>01:00:00:00</string>
            <frame>86400</frame>  <!-- <====THIS LINE-->
            <displayformat>NDF</displayformat>
        </timecode>
        ```

    # Examples

    ```rust
    # use vtc::{Timecode, rates};
    let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    assert_eq!(86400, tc.frames())
    ```
    */
    pub fn frames(&self) -> i64 {
        let rational_frames = self.seconds * self.rate.playback();
        if rational_frames.denom() == &1 {
            return *rational_frames.numer();
        };

        rational_frames.round().to_integer()
    }

    /**
    Returns the true, real-world runtime of the timecode in HH:MM:SS.FFFFFFFFF format.

    # Arguments

    * `precision` - How many places to print after the decimal. Tailing 0's will be truncated
      regardless of setting.

    # What it is

    The formatted version of seconds. It looks like timecode, but with a decimal seconds value
    instead of a frame number place.

    # Where you see it

    - Anywhere real-world time is used.
    - FFMPEG commands:

        ```shell
        ffmpeg -ss 00:00:30.5 -i input.mov -t 00:00:10.25 output.mp4
        ```

    # Examples

    ```rust
    # use vtc::{Timecode, rates};
    let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    assert_eq!("01:00:03.6", tc.runtime(9))
    ```

    ## note:

    Runtime and timecode will differ with NTSC framerates. NTSC reports timecode *as-if* it
    were running at a whole-frame rate (so 23.98 is reported as if it were running at 24.)

    [Timecode::runtime] reports the true, real-world time elapsed since 00:00:00:00.
    */
    pub fn runtime(&self, precision: usize) -> String {
        // We use the absolute seconds here so floor behaves as expected regardless of whether
        // this value is negative.
        let mut seconds = abs(self.seconds);
        let hours = (seconds / SECONDS_PER_HOUR).floor().to_integer();
        seconds %= SECONDS_PER_HOUR;

        let minutes = (seconds / SECONDS_PER_MINUTE).floor().to_integer();
        seconds %= SECONDS_PER_MINUTE;

        let fract = seconds.fract();
        let seconds_int = seconds.floor().to_integer();

        let fract_str = if fract == Rational64::zero() {
            ".0".to_string()
        } else {
            let formatted = format!("{:.1$}", fract.to_f64().unwrap_or(0.0), precision);
            let mut formatted = formatted.trim_start_matches('0');
            formatted = formatted.trim_end_matches('0');
            formatted.to_string()
        };

        let sign = if self.seconds.is_negative() { "-" } else { "" };

        format!(
            "{}{:02}:{:02}:{:02}{}",
            sign, hours, minutes, seconds_int, fract_str,
        )
    }

    /**
    Returns the number of elapsed ticks this timecode represents in Adobe Premiere Pro.

    # What it is

    Internally, Adobe Premiere Pro uses ticks to divide up a second, and keep track of how far into
    that second we are. There are 254016000000 ticks in a second, regardless of framerate in
    Premiere.

    # Where you see it

    - Premiere Pro Panel functions and scripts
    - FCP7XML cutlists generated from Premiere:

        ```xml
        <clipitem id="clipitem-1">
            ...
            <in>158</in>
            <out>1102</out>
            <pproTicksIn>1673944272000</pproTicksIn>
            <pproTicksOut>11675231568000</pproTicksOut>
            ...
        </clipitem>
        ```

    # Examples

    ```rust
    # use vtc::{Timecode, rates};
    use num::Rational64;
    let tc = Timecode::with_seconds(1.0, rates::F24).unwrap();
    assert_eq!(254016000000, tc.premiere_ticks())
    ```

    ```rust
    # use vtc::{Timecode, rates};
    let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    assert_eq!(915372057600000, tc.premiere_ticks())
    ```
    */
    pub fn premiere_ticks(&self) -> i64 {
        // We need to jump up to a i128-based rat for a second to avoid an overflow
        // here.
        let seconds128 =
            Ratio::<i128>::new(*self.seconds.numer() as i128, *self.seconds.denom() as i128);

        let seconds_int = (seconds128 * PREMIERE_TICKS_PER_SECOND)
            .round()
            .to_integer();

        seconds_int as i64
    }

    /**
    Create a feet+frames string of `self`.

    # What it is

    The customary text representation of frame-accurate film footage.

    The `rep` parameter selects the film format. 35mm, 4-perf vertical
    pulldown is by far the most common, but other film gauges and pulldowns
    are supported. See the [FeetFramesRep] enum for all available formats.

    # Where you see it

    - Film assembly and pull lists, chronologs and camera reports
    - FLX and ALE telecine log files
    - Avid film pull lists, cut lists and change lists

    */
    pub fn feet_and_frames(&self, rep: FilmFormat) -> String {
        // get the frame count and sign
        let total_frames = abs(self.frames());
        let negative = self.seconds.is_negative();

        let total_perfs = total_frames * rep.perfs_per_frame();

        // The foot a frame lies in, by convention, is where the frame _ends_,
        // not where it begins. If we add rep.perfs_per_frame() -1 to the perf
        // count, we get the last perf of the frame we are on.
        //
        // Once we have that, the foot we are on can be obtained with integer
        // division.
        let last_perf = total_perfs + rep.perfs_per_frame() - 1;
        let feet = last_perf / rep.perfs_per_foot();

        // The perf marker is simply the modulo of the footage count and the 
        // number of feet in a footage modulus.
        //
        // For almost all formats this is always zero, but not in the case of
        // 3-perf.
        //
        // The meaning of the perf marker is obscure but it indicates the
        // position of the Kodak KeyKode "black dot," a dot that appears
        // every 32 perfs (6 inches) and witnesses an asociated KeyKode latent 
        // edge number. On an Avid the perf marker is represented as a number, 
        // However on window dubs from Evertz TCGs this would appear as a visual
        // symbol,
        let perf_marker = feet % rep.footage_modulus_footage_count();

        let frames = (last_perf % rep.perfs_per_foot()) / rep.perfs_per_frame();

        let sign = if negative { "-" } else { "" };

        if rep.allows_perf_field() {
            format!("{}{}+{:02}.{}", sign, feet, frames, perf_marker)
        } else {
            format!("{}{}+{:02}", sign, feet, frames)
        }
    }

    /// Returns a [Timecode] with the same number of frames running at a different
    /// [Framerate].
    ///
    /// # Arguments
    ///
    /// * `rate` - The new framerate to apply to the frrame count..
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_frames("01:00:00:00", rates::F24).unwrap();
    /// let rebased = tc.rebase(rates::F48);
    /// assert_eq!("00:30:00:00", rebased.timecode())
    /// ```
    pub fn rebase(&self, rate: Framerate) -> Self {
        Timecode::with_i64_frames(self.frames(), rate)
    }

    /// Returns the absolute value of the [Timecode] value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// use num::Rational64;
    /// let tc = Timecode::with_frames("-01:00:00:00", rates::F23_98).unwrap();
    /// assert_eq!("01:00:00:00", tc.abs().timecode())
    /// ```
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    /// assert_eq!("01:00:00:00", tc.abs().timecode())
    /// ```
    pub fn abs(&self) -> Self {
        Timecode::with_rational_seconds(abs(self.seconds), self.rate)
    }

    /// Returns a new [Timecode] with a [Timecode::frames] return value equal to the frames arg.
    ///
    /// [Timecode::with_frames] takes many different formats (more than just numeric types) that
    /// represent the frame count of the timecode.
    ///
    /// # Arguments
    ///
    /// * `frames` - A value which can be represented as a frame number / frame count.
    ///
    /// * `rate` - The Framerate at which the frames are being played back.
    ///
    /// # Examples
    ///
    /// Create a [Timecode] from a timecode string:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    ///
    /// From am integer frame count:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_frames(86400, rates::F23_98).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    ///
    /// From a feet+frames string:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// use num::Rational64;
    /// let tc = Timecode::with_frames("5400+00", rates::F23_98).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    pub fn with_frames<T: FramesSource>(frames: T, rate: Framerate) -> TimecodeParseResult {
        let frame_count = frames.to_frames(rate)?;
        Ok(Self::with_i64_frames(frame_count, rate))
    }

    /// Returns a new [Timecode] with a [Timecode::seconds] return value equal to the seconds arg
    /// (rounded to the nearest frame).
    ///
    /// [Timecode::with_seconds] takes many different formats (more than just numeric types) that
    /// represent the frame count of the timecode.
    ///
    /// # Arguments
    ///
    /// * `seconds` - A value which can be represented as a number of seconds.
    ///
    /// * `rate` - The Framerate which seconds will be rounded to match the nearest frame with.
    ///
    /// # Examples
    ///
    /// From a float value:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_seconds(3600.0, rates::F24).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    ///
    /// From a Rational64 value:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// use num::Rational64;
    /// let tc = Timecode::with_seconds(Rational64::new(3600, 1), rates::F24).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    ///
    /// From a Rational64 runtime:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// let tc = Timecode::with_seconds("01:00:00.0", rates::F24).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    ///
    /// ## Note:
    ///
    /// Remember that seconds are rounded to the nearest whole frame, so what you get back may not
    /// exactly match what you put in:
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// use num::Rational64;
    /// let tc = Timecode::with_seconds(Rational64::new(3600, 1), rates::F23_98).unwrap();
    /// assert_eq!(Rational64::new(43200157, 12000), tc.seconds())
    /// ```
    pub fn with_seconds<T: SecondsSource>(seconds: T, rate: Framerate) -> TimecodeParseResult {
        let seconds_rat = seconds.to_seconds(rate)?;
        Ok(Self::with_rational_seconds(seconds_rat, rate))
    }

    /// Returns a new [Timecode] with a [Timecode::premiere_ticks] return value equal to the ticks
    /// arg.
    ///
    /// # Arguments
    ///
    /// * `ticks` - A value which can be represented as a number Adobe Premiere Pro ticks.
    ///
    /// * `rate` - The Framerate which seconds will be rounded to match the nearest frame with.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use vtc::{Timecode, rates};
    /// use num::Rational64;
    /// let tc = Timecode::with_premiere_ticks(915372057600000i64, rates::F23_98).unwrap();
    /// assert_eq!("01:00:00:00", tc.timecode())
    /// ```
    pub fn with_premiere_ticks<T: PremiereTicksSource>(
        ticks: T,
        rate: Framerate,
    ) -> TimecodeParseResult {
        let tick_count = ticks.to_ticks(rate)?;
        // We need to do this calculation in a 128-bit Ratio because otherwise
        // PREMIERE_TICKS_PER_SECOND could easily cause an integer overflow for a reasonably i64
        // seconds value.
        let seconds128 =
            Ratio::<i128>::from_integer(tick_count as i128) / PREMIERE_TICKS_PER_SECOND;
        let seconds = Rational64::new(*seconds128.numer() as i64, *seconds128.denom() as i64);
        Self::with_seconds(seconds, rate)
    }

    /// Used internally for creating new timecodes from i64 frame count values without
    /// an error return.
    fn with_i64_frames(frame_count: i64, rate: Framerate) -> Timecode {
        let seconds = Rational64::from_integer(frame_count) / rate.playback();
        Self::with_rational_seconds(seconds, rate)
    }

    /// Used internally for creating new timecodes from Rational64 seconds values
    /// without an error return.
    fn with_rational_seconds(seconds: Rational64, rate: Framerate) -> Timecode {
        let seconds = round_seconds_to_frame(seconds, rate);
        Timecode { seconds, rate }
    }
}

impl Display for Timecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} @ {}]", self.timecode(), self.rate)
    }
}

impl PartialEq for Timecode {
    fn eq(&self, other: &Self) -> bool {
        self.seconds == other.seconds
    }
}

impl Eq for Timecode {}

impl PartialOrd for Timecode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.seconds.partial_cmp(&other.seconds)
    }
}

impl Ord for Timecode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seconds.cmp(&other.seconds)
    }
}

impl Add for Timecode {
    type Output = Timecode;

    fn add(self, rhs: Self) -> Self::Output {
        let new_seconds = self.seconds + rhs.seconds;
        Timecode::with_rational_seconds(new_seconds, self.rate())
    }
}

impl<T> AddAssign<T> for Timecode
where
    Timecode: Add<T, Output = Timecode>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl Sub for Timecode {
    type Output = Timecode;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_seconds = self.seconds - rhs.seconds;
        Timecode::with_rational_seconds(new_seconds, self.rate)
    }
}

impl<T> SubAssign<T> for Timecode
where
    Timecode: Sub<T, Output = Timecode>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs
    }
}

impl Mul<Rational64> for Timecode {
    type Output = Timecode;

    fn mul(self, rhs: Rational64) -> Self::Output {
        let new_seconds = self.seconds * rhs;
        Timecode::with_rational_seconds(new_seconds, self.rate)
    }
}

impl Mul<f64> for Timecode {
    type Output = Timecode;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs_rat = Rational64::from_f64(rhs).unwrap();
        let new_seconds = self.seconds * rhs_rat;
        Timecode::with_rational_seconds(new_seconds, self.rate)
    }
}

impl Mul<Timecode> for f64 {
    type Output = Timecode;

    fn mul(self, rhs: Timecode) -> Self::Output {
        rhs * self
    }
}

impl Mul<i64> for Timecode {
    type Output = Timecode;

    fn mul(self, rhs: i64) -> Self::Output {
        let rhs_rat = Rational64::from_integer(rhs);
        let new_seconds = self.seconds * rhs_rat;
        Timecode::with_rational_seconds(new_seconds, self.rate)
    }
}

impl Mul<Timecode> for i64 {
    type Output = Timecode;

    fn mul(self, rhs: Timecode) -> Self::Output {
        rhs * self
    }
}

impl<T> MulAssign<T> for Timecode
where
    Timecode: Mul<T, Output = Timecode>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl Div<Rational64> for Timecode {
    type Output = Timecode;

    fn div(self, rhs: Rational64) -> Self::Output {
        let mut frames_rat = Rational64::from_integer(self.frames());
        frames_rat /= rhs;
        frames_rat = frames_rat.floor();
        Timecode::with_i64_frames(frames_rat.to_integer(), self.rate)
    }
}

impl Rem<Rational64> for Timecode {
    type Output = Timecode;

    fn rem(self, rhs: Rational64) -> Self::Output {
        let mut frames_rat = Rational64::from_integer(self.frames());
        frames_rat %= rhs;
        frames_rat = frames_rat.round();
        Timecode::with_i64_frames(frames_rat.to_integer(), self.rate)
    }
}

impl Div<f64> for Timecode {
    type Output = Timecode;

    fn div(self, rhs: f64) -> Self::Output {
        // We're going to do the actual operation with rationals.
        let rhs_rat = Rational64::from_f64(rhs).unwrap();
        self / rhs_rat
    }
}

impl Rem<f64> for Timecode {
    type Output = Timecode;

    fn rem(self, rhs: f64) -> Self::Output {
        let rhs_rat = Rational64::from_f64(rhs).unwrap();
        self % rhs_rat
    }
}

impl Div<i64> for Timecode {
    type Output = Timecode;

    fn div(self, rhs: i64) -> Self::Output {
        let frames_divided = self.frames() / rhs;
        Timecode::with_i64_frames(frames_divided, self.rate)
    }
}

impl Rem<i64> for Timecode {
    type Output = Timecode;

    fn rem(self, rhs: i64) -> Self::Output {
        let frames_remainder = self.frames() % rhs;
        Timecode::with_i64_frames(frames_remainder, self.rate)
    }
}

impl<T> DivAssign<T> for Timecode
where
    Timecode: Div<T, Output = Timecode>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T> RemAssign<T> for Timecode
where
    Timecode: Rem<T, Output = Timecode>,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl Neg for Timecode {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Timecode::with_rational_seconds(-self.seconds, self.rate)
    }
}

/// Converts a frame-number to an adjusted frame number for creating drop-frame tc.
///
/// Algorithm adapted from: https://www.davidheidelberger.com/2010/06/10/drop-frame-timecode/
///
/// * `frame_number` - The frame number to convert to a drop-frame number.
///
/// * `rate` - the framerate of the timecode.
///
/// returns The frame number adjusted to produce the correct drop-frame timecode when
/// used in the normal timecode calculation.
///
/// ***WARNING:*** This method will panic if passed a non-drop-frame framerate.
fn drop_frame_adjustment(mut frame_number: i64, rate: Framerate) -> i64 {
    let framerate = rate.playback().to_f64().unwrap();

    let dropped_per_min = rate.drop_frames_per_minute().unwrap();
    let frames_per_hour = (framerate * 60.0 * 60.0).round().to_i64().unwrap();
    let frames_per_24_hours = frames_per_hour * 24;
    let frames_per_10_min = (framerate * 60.0 * 10.0).round().to_i64().unwrap();
    let frames_per_min = (framerate * 60.0).round().to_i64().unwrap();

    frame_number %= frames_per_24_hours;

    let (tens_of_mins, remaining_mins) = div_rem(frame_number, frames_per_10_min);
    let tens_of_mins_adjustment = dropped_per_min * 9 * tens_of_mins;

    if remaining_mins > dropped_per_min {
        let remaining_minutes_adjustment =
            dropped_per_min * (remaining_mins - dropped_per_min) / frames_per_min;

        tens_of_mins_adjustment + remaining_minutes_adjustment
    } else {
        tens_of_mins_adjustment
    }
}
