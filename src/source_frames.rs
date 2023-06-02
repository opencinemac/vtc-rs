use num::integer::div_floor;
use num::Rational64;
use regex::Match;
use std::convert::TryFrom;
use std::fmt::Debug;

use crate::consts::{
    FEET_AND_FRAMES_REGEX, SECONDS_PER_HOUR_I64, SECONDS_PER_MINUTE_I64, TIMECODE_REGEX,
};
use crate::{
    timecode_parse, FeetFramesStr, FilmFormat, Framerate, Ntsc, TimecodeParseError,
    TimecodeSections,
};

/// The result type of [FramesSource::to_frames].
pub type FramesSourceResult = Result<i64, TimecodeParseError>;

/// Types implementing this trait can be converted into the number of frames that have elapsed since
/// a timecode value of 00:00:00:00.
pub trait FramesSource: Debug {
    /// Returns the number of frames this value represents.
    fn to_frames(&self, rate: Framerate) -> FramesSourceResult;
}

impl<T> FramesSource for &T
where
    T: FramesSource,
{
    fn to_frames(&self, rate: Framerate) -> FramesSourceResult {
        (*self).to_frames(rate)
    }
}

impl FramesSource for &dyn FramesSource {
    fn to_frames(&self, rate: Framerate) -> FramesSourceResult {
        (*self).to_frames(rate)
    }
}

impl FramesSource for i64 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(*self)
    }
}

impl FramesSource for isize {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        let i64_val = match i64::try_from(*self) {
            Ok(converted) => converted,
            Err(err) => {
                return Err(TimecodeParseError::Conversion(format!(
                    "error converting isize to i64 : {}",
                    err
                )))
            }
        };

        Ok(i64_val)
    }
}

impl FramesSource for usize {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        let i64_val = match i64::try_from(*self) {
            Ok(converted) => converted,
            Err(err) => {
                return Err(TimecodeParseError::Conversion(format!(
                    "error converting usize to i64 : {}",
                    err
                )))
            }
        };

        Ok(i64_val)
    }
}

impl FramesSource for u64 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        let i64_val = match i64::try_from(*self) {
            Ok(converted) => converted,
            Err(err) => {
                return Err(TimecodeParseError::Conversion(format!(
                    "error converting u64 to i64 : {}",
                    err
                )))
            }
        };

        Ok(i64_val)
    }
}

impl FramesSource for i32 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(i64::from(*self))
    }
}

impl FramesSource for u32 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(i64::from(*self))
    }
}

impl FramesSource for i16 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(i64::from(*self))
    }
}

impl FramesSource for u16 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(i64::from(*self))
    }
}

impl FramesSource for i8 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(i64::from(*self))
    }
}

impl FramesSource for u8 {
    fn to_frames(&self, _: Framerate) -> FramesSourceResult {
        Ok(i64::from(*self))
    }
}

impl FramesSource for &str {
    fn to_frames(&self, rate: Framerate) -> FramesSourceResult {
        if let Some(matched) = TIMECODE_REGEX.captures(self) {
            return parse_timecode_string(matched, rate);
        }

        if let Some(matched) = FEET_AND_FRAMES_REGEX.captures(self) {
            return parse_feet_and_frames_str(matched, None);
        }

        Err(TimecodeParseError::UnknownStrFormat(format!(
            "{} is not a known frame-count timecode format",
            self
        )))
    }
}

impl<'a> FramesSource for FeetFramesStr<'a> {
    fn to_frames(&self, _rate: Framerate) -> FramesSourceResult {
        if let Some(matched) = FEET_AND_FRAMES_REGEX.captures(self.input) {
            parse_feet_and_frames_str(matched, Some(self.format))
        } else {
            Err(TimecodeParseError::UnknownStrFormat(format!(
                "{} is not a known frame-count timecode format",
                self.input
            )))
        }
    }
}

impl FramesSource for String {
    fn to_frames(&self, rate: Framerate) -> FramesSourceResult {
        self.as_str().to_frames(rate)
    }
}

/// parse_timecode_string parses a tc string matched by TIMECODE_REGEX into a frame count.
fn parse_timecode_string(matched: regex::Captures, rate: Framerate) -> FramesSourceResult {
    // We can unwrap the frames here because we know that if the regex matched, the frames value
    // must be there.
    let frames =
        timecode_parse::convert_tc_int(matched.name("frames").unwrap().as_str(), "frames")?;

    // We need to figure out how many other sections were present. We'll put them into this vec.
    let mut sections: Vec<Match> = Vec::new();
    if let Some(section) = matched.name("section1") {
        sections.push(section);
    };
    if let Some(section) = matched.name("section2") {
        sections.push(section);
    };
    if let Some(section) = matched.name("section3") {
        sections.push(section);
    };

    // Get whether this value was a negative timecode value.
    let is_negative = matched.name("negative").is_some();

    // Start popping values and assigning them moving seconds -> hours to account for partial
    // timecode values like '1:12'. Fill in 0 on missing sections.
    let seconds: i64 = match sections.pop() {
        None => 0,
        Some(section) => timecode_parse::convert_tc_int(section.as_str(), "seconds")?,
    };

    let minutes: i64 = match sections.pop() {
        None => 0,
        Some(section) => timecode_parse::convert_tc_int(section.as_str(), "minutes")?,
    };

    let hours: i64 = match sections.pop() {
        None => 0,
        Some(section) => timecode_parse::convert_tc_int(section.as_str(), "frames")?,
    };

    // Get the drop-frame adjustment.
    let drop_adjustment = if rate.ntsc() == Ntsc::DropFrame {
        drop_frame_tc_adjustment(
            TimecodeSections {
                negative: is_negative,
                hours,
                minutes,
                seconds,
                frames,
            },
            rate,
        )?
    } else {
        0
    };

    // Get the total seconds from the seconds, minutes, and hours.
    let seconds = seconds + minutes * SECONDS_PER_MINUTE_I64 + hours * SECONDS_PER_HOUR_I64;
    // Convert our seconds and frames to a frames count by multiplying seconds by the timebase and
    // adding the remaining frames.
    let frames_rat =
        Rational64::from_integer(seconds) * rate.timebase() + Rational64::from_integer(frames);

    let mut frames = frames_rat.round().to_integer();
    frames += drop_adjustment;
    if is_negative {
        frames *= -1
    }

    Ok(frames)
}

/// adjusts the frame number based on drop-frame TC conventions.
///
/// Algorithm adapted from:
/// https://www.davidheidelberger.com/2010/06/10/drop-frame-timecode/
///
/// **WARNING** this method will panic if a non-drop-frame Framerate is passed to it.
fn drop_frame_tc_adjustment(sections: TimecodeSections, rate: Framerate) -> FramesSourceResult {
    // Get the number of frames we need to drop each time we drop frames (ex: 2 for 29.97)
    let drop_frames = rate.drop_frames_per_minute().unwrap();

    // We have a bad frame value if our 'frames' place is less than the drop_frames we
    // skip on minutes not divisible by 10.
    let has_bad_frames = sections.frames < drop_frames;
    let is_tenth_minute = sections.minutes % 10 == 0;
    let is_minute_boundary = sections.seconds == 0;

    if has_bad_frames && is_minute_boundary && !is_tenth_minute {
        return Err(TimecodeParseError::DropFrameValue(format!(
            "drop-frame tc cannot have a frames value of less than {} on minutes not divisible by 10, found '{}'",
            drop_frames,
            sections.frames,
        )));
    };

    let total_minutes = 60 * sections.hours + sections.minutes;
    // calculate the adjustment, we need to remove two frames for each minute except for every
    // 10th minute.
    let adjustment = drop_frames * (total_minutes - total_minutes / 10);

    // We need the adjustment to remove frames, so return a negative.
    Ok(-adjustment)
}

fn parse_feet_and_frames_str(
    matched: regex::Captures,
    given_format: Option<FilmFormat>,
) -> FramesSourceResult {
    // If we got a match, these groups had to be present, so we can unwrap them.

    let feet = timecode_parse::convert_tc_int(matched.name("feet").unwrap().as_str(), "feet")?;

    let frames =
        timecode_parse::convert_tc_int(matched.name("frames").unwrap().as_str(), "frames")?;

    // Parse perfs field if it was present otherwise pull a Option::None.
    let perfs_n = matched
        .name("perf")
        .and_then(|perfs_n| perfs_n.as_str().parse::<i64>().ok());

    // Get whether this value was a negative timecode value.
    let is_negative = matched.name("negative").is_some();

    // Infer the format of the footage if it hasn't been provided.
    let final_format : Result<FilmFormat, TimecodeParseError> = match (given_format, perfs_n) {
        (Some(film_format) , Some(_)) if !film_format.allows_perf_field()  => Err(TimecodeParseError::UnknownStrFormat(
             format!("Perf field was present in string \"{}\", which is not allowed for given film format {:?}.", 
                 matched.get(0).unwrap().as_str(), film_format)
             )
            ),
        (Some(film_format), _) => Ok(film_format),
        (None, Some(_)) => Ok(FilmFormat::FF35mm3perf),
        (_, _) => Ok(FilmFormat::FF35mm4perf),
    };

    if let Ok(final_format) = final_format {
        // If the number of perfs in a foot is evenly divisible in perfs in a frame,
        // this will be the same as the final footage count. If not (as in 35mm 3 perf),
        // there will be a couple feet left over.

        // We set up `rem_frames` with `frames` from the string. This will accumulate
        // our final result.
        let mut rem_frames = frames;

        // We obtain the count of integral footage moduli in the `feet` count with floor
        // division.
        let footage_moduli = div_floor(feet, final_format.footage_modulus_footage_count());

        // There may be feet left over, because we took the floor value.
        let mut rem_feet = feet - (footage_moduli * final_format.footage_modulus_footage_count());

        // Add all the frames in the footage_moduli.
        rem_frames += footage_moduli * final_format.footage_modulus_frame_count();

        // If there WEREN'T any feet left over, we can just continue, but if there were,
        // we have to step through each remaining foot in the modulus and add the
        // leftover frames in those feet to rem_frames.
        while rem_feet > 0 {
            rem_frames += final_format.footage_modulus_frame_count()
                / final_format.footage_modulus_footage_count();
            rem_feet -= 1;
        }

        // Negate if indicated.
        if is_negative {
            rem_frames = -rem_frames;
        };

        // We divide by perfs per frame to obtain the final frame count value
        Ok(dbg!(rem_frames))
    } else {
        Err(final_format.err().unwrap())
    }
}
