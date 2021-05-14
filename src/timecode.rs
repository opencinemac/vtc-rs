use num::integer::div_mod_floor;
use num::traits::Inv;
use num::{abs, Rational64, Signed, ToPrimitive, Zero};

use crate::consts::{
    FRAMES_PER_FOOT, PREMIERE_TICKS_PER_SECOND, SECONDS_PER_HOUR, SECONDS_PER_MINUTE,
};
use crate::source_ppro_ticks::PremiereTicksSource;
use crate::{Framerate, FramesSource, Ntsc, SecondsSource, TimecodeParseError};

/// Holds the individual sections of a timecode for formatting / manipulation.
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

/// The [Result] type returned by [Timecode::new_with_seconds] and [Timecode::new_with_frames].
pub type TimecodeParseResult = Result<Timecode, TimecodeParseError>;

#[derive(Clone, Copy, Debug)]
pub struct Timecode {
    seconds: Rational64,
    rate: Framerate,
}

impl Timecode {
    /// returns the Framerate of the timecode.
    pub fn rate(&self) -> Framerate {
        self.rate
    }

    /// returns the rational representation of the real-world seconds that would have elapsed
    /// between 00:00:00:00 and this timecode.
    pub fn seconds(&self) -> Rational64 {
        self.seconds
    }

    /// The individual sections of a timecode string as i64 values.
    pub fn sections(&self) -> TimecodeSections {
        // We use the absolute frame count here so floor behaves as expected regardless of whether
        // this value is negative.
        let mut frames = Rational64::from(abs(self.frames()));
        let timebase = self.rate.timebase();

        if self.rate.ntsc() == Ntsc::DropFrame {
            // Convert the frame number to an adjusted one for drop-frame display values.
            let frames_int = frame_num_to_drop_num(frames.to_integer(), self.rate);
            frames = Rational64::from_integer(frames_int)
        }

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

    /// returns the the formatted SMPTE timecode: (ex: 01:00:00:00).
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

    /// returns the number of frames that would have elapsed between 00:00:00:00 and this timecode.
    pub fn frames(&self) -> i64 {
        let rational_frames = self.seconds * self.rate.playback();
        if rational_frames.denom() == &1 {
            return *rational_frames.numer();
        };

        rational_frames.round().to_integer()
    }

    /// returns the true runtime of the timecode in HH:MM:SS.FFFFFFFFF format.
    pub fn runtime(&self, precision: usize) -> String {
        // We use the absolute seconds here so floor behaves as expected regardless of whether
        // this value is negative.
        let hours = (abs(self.seconds) / SECONDS_PER_HOUR).floor().to_integer();
        let mut seconds = self.seconds % SECONDS_PER_HOUR;

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

    /// Returns the number of elapsed ticks this timecode represents in Adobe Premiere Pro.
    ///
    /// Premiere uses ticks internally to track elapsed time. A second contains 254016000000 ticks,
    /// regardless of framerate.
    ///
    /// These ticks are present in Premiere FCP7XML cutlists, and can sometimes be used for more
    /// precise calculations during respeeds.
    ///
    /// Ticks are also used for scripting in Premiere Panels.
    pub fn premiere_ticks(&self) -> i64 {
        (self.seconds * PREMIERE_TICKS_PER_SECOND)
            .round()
            .to_integer()
    }

    /// Returns the number of feet and frames this timecode represents if it were shot on 35mm
    /// 4-perf film (16 frames per foot). ex: '5400+13'.
    ///
    /// Feet and frames is most commonly used as a reference in the sound mixing world.
    pub fn feet_and_frames(&self) -> String {
        let result = div_mod_floor(abs(self.frames()), FRAMES_PER_FOOT);
        let feet = result.0;
        let frames = result.1;

        let sign = if self.seconds.is_negative() { "-" } else { "" };

        return format!("{}{}+{:02}", sign, feet, frames);
    }

    /// Returns a new [Timecode] with a [Timecode::frames] return value equal to the frames arg.
    pub fn new_with_frames<T: FramesSource>(frames: T, rate: Framerate) -> TimecodeParseResult {
        let frame_count = frames.to_frames(rate)?;
        let seconds = Rational64::from_integer(frame_count) / rate.playback();
        Self::new_with_seconds(&seconds, rate)
    }

    /// Returns a new [Timecode] with a [Timecode::seconds] return value equal to the seconds arg
    /// (rounded to the nearest frame).
    pub fn new_with_seconds<T: SecondsSource>(seconds: T, rate: Framerate) -> TimecodeParseResult {
        let mut seconds_rat = seconds.to_seconds(rate)?;

        // if our value can be cleanly divied by the length of a single frame, we can use it as-is.
        seconds_rat = if seconds_rat == num::Rational64::zero() % rate.playback().inv() {
            let frames = (seconds_rat * rate.playback()).round();
            frames / rate.playback()
        } else {
            seconds_rat
        };

        Ok(Timecode {
            seconds: seconds_rat,
            rate,
        })
    }

    /// Returns a new [Timecode] with a [Timecode::premiere_ticks] return value equal to the ticks
    /// arg.
    pub fn new_with_premiere_ticks<T: PremiereTicksSource>(
        ticks: T,
        rate: Framerate,
    ) -> TimecodeParseResult {
        let tick_count = ticks.to_ticks(rate)?;
        let seconds = Rational64::from_integer(tick_count) / PREMIERE_TICKS_PER_SECOND;
        Self::new_with_seconds(&seconds, rate)
    }
}

impl PartialEq for Timecode {
    fn eq(&self, other: &Self) -> bool {
        self.seconds == other.seconds
    }
}

impl Eq for Timecode {}

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
fn frame_num_to_drop_num(frame_number: i64, rate: Framerate) -> i64 {
    // Get the timebase as an i64. NTSC timebases are always whole-frame.
    let timebase = rate.timebase().to_integer();

    // Get the number frames-per-minute at the whole-frame rate.
    let frames_per_minute = timebase * 60;
    // Get the number of frames we need to drop each time we drop frames (ex: 2 or 29.97).
    let drop_frames = rate.drop_frames().unwrap();

    // Get the number of frames are in a minute where we have dropped frames at the
    // beginning.
    let frames_per_minute_drop = (timebase * 60) - drop_frames;
    // Get the number of actual frames in a 10-minute span for drop frame timecode. Since
    // we drop 9 times a minute, it will be 9 drop-minute frame counts + 1 whole-minute
    // frame count.
    let frames_per_10minutes_drop = frames_per_minute_drop * 9 + frames_per_minute;

    // Get the number of 10s of minutes in this count, and the remaining frames.
    let result = div_mod_floor(frame_number, frames_per_10minutes_drop);
    let tens_of_minutes = result.0;
    let mut frames = result.1;

    // Create an adjustment for the number of 10s of minutes. It will be 9 times the
    // drop value (we drop for the first 9 minutes, then leave the 10th alone).
    let mut adjustment = 9 * drop_frames * tens_of_minutes;

    // If our remaining frames are less than a whole minute, we aren't going to drop
    // again. Add the adjustment and return.
    if frames < frames_per_minute {
        return frame_number + adjustment;
    };

    // Remove the first full minute (we don't drop until the next minute) and add the
    // drop-rate to the adjustment.
    frames -= timebase;
    adjustment += drop_frames;

    // Get the number of remaining drop-minutes present, and add a drop adjustment for
    // each.
    let minutes_drop = frames / frames_per_minute_drop;
    adjustment += minutes_drop * drop_frames;

    // Return our original frame number adjusted by our calculated adjustment.
    frame_number + adjustment
}
