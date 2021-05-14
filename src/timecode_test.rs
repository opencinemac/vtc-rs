#[cfg(test)]
mod test {
    use num::{Rational64, ToPrimitive};
    use rstest::rstest;

    use crate::{
        rates, source_ppro_ticks::PremiereTicksSource, Framerate, Timecode, TimecodeParseError,
    };
    use crate::{source_frames::FramesSource, SecondsSource};
    use std::fmt::Debug;
    use std::ops::Deref;

    struct ParseCase {
        frames_sources: Vec<Box<dyn FramesSource>>,
        seconds_sources: Vec<Box<dyn SecondsSource>>,
        ticks_sources: Vec<Box<dyn PremiereTicksSource>>,
        rate: Framerate,
        seconds: Rational64,
        frames: i64,
        timecode: String,
        runtime: String,
        feet_and_frames: String,
        premiere_ticks: i64,
    }

    #[rstest]
    // 23.98 NTSC CASES ----------
    // ---------------------------
    #[case(
        ParseCase{
            frames_sources: vec![
                Box::new("00:40:00:00".to_string()),
                Box::new("40:00:00".to_string()),
                Box::new("57600".to_string()),
                Box::new("3600+00".to_string()),
                Box::new(57600i64),
                Box::new(57600u64),
                Box::new(57600i32),
                Box::new(57600u32),
                Box::new(57600isize),
                Box::new(57600usize),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(12012, 5)),
                Box::new(Rational64::new(12012, 5).to_f64().unwrap()),
                Box::new(Rational64::new(12012, 5).to_f32().unwrap()),
                Box::new("00:40:02.4".to_string()),
                Box::new("2402.4".to_string()),
            ],
            ticks_sources: vec![
                Box::new(610248038400000i64),
                Box::new(610248038400000u64),
            ],
            rate: rates::F23_98,
            seconds: Rational64::new(12012, 5),
            frames: 57600,
            timecode: "00:40:00:00".to_string(),
            runtime: "00:40:02.4".to_string(),
            feet_and_frames: "3600+00".to_string(),
            premiere_ticks: 610248038400000,
        }
    )]
    // 24 CASES ------------------
    // ---------------------------
    #[case::t01_00_00_00_f24(
        ParseCase{
            frames_sources: vec![
                Box::new("01:00:00:00".to_string()),
                Box::new("1:00:00:00".to_string()),
                Box::new("86400".to_string()),
                Box::new("5400+00".to_string()),
                Box::new(86400i64),
                Box::new(86400u64),
                Box::new(86400i32),
                Box::new(86400u32),
                Box::new(86400isize),
                Box::new(86400usize),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(3600, 1)),
                Box::new(Rational64::new(3600, 1).to_f64().unwrap()),
                Box::new(Rational64::new(3600, 1).to_f32().unwrap()),
                Box::new("01:00:00.0".to_string()),
                Box::new("01:00:00".to_string()),
                Box::new("3600.0".to_string()),
            ],
            ticks_sources: vec![
                Box::new(914457600000000i64),
                Box::new(914457600000000u64),
            ],
            rate: rates::F24,
            seconds: Rational64::new(3600, 1),
            frames: 86400,
            timecode: "01:00:00:00".to_string(),
            runtime: "01:00:00.0".to_string(),
            feet_and_frames: "5400+00".to_string(),
            premiere_ticks: 914457600000000,
        }
    )]
    // 29.96 DF CASES ------------
    // ---------------------------
    #[case::t00_00_01_02_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:01:00;02".to_string()),
                Box::new("01:00;02".to_string()),
                Box::new("1:00;02".to_string()),
                Box::new("1800".to_string()),
                Box::new("112+08".to_string()),
                Box::new(1800i64),
                Box::new(1800u64),
                Box::new(1800i32),
                Box::new(1800u32),
                Box::new(1800i16),
                Box::new(1800u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(3003, 50)),
                Box::new(Rational64::new(3003, 50).to_f64().unwrap()),
                Box::new(Rational64::new(3003, 50).to_f32().unwrap()),
                Box::new("00:01:00.06".to_string()),
                Box::new("1:00.06".to_string()),
                Box::new("60.06".to_string()),
            ],
            ticks_sources: vec![
                Box::new(15256200960000i64),
                Box::new(15256200960000u64),
            ],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(3003, 50),
            frames: 1800,
            timecode: "00:01:00;02".to_string(),
            runtime: "00:01:00.06".to_string(),
            feet_and_frames: "112+08".to_string(),
            premiere_ticks: 15256200960000,
        }
    )]
    fn test_parse_timecode(#[case] case: ParseCase) -> Result<(), TimecodeParseError> {
        for boxed in case.frames_sources.iter() {
            let source = boxed.deref();
            let tc = Timecode::new_with_frames(source, case.rate)?;
            check_parsed(&case, tc, source, "frames")
        }

        for boxed in case.seconds_sources.iter() {
            let source = boxed.deref();
            let tc = Timecode::new_with_seconds(source, case.rate)?;
            check_parsed(&case, tc, source, "seconds")
        }

        for boxed in case.ticks_sources.iter() {
            let source = boxed.deref();
            let tc = Timecode::new_with_premiere_ticks(source, case.rate)?;
            check_parsed(&case, tc, source, "ppro_ticks")
        }

        Ok(())
    }

    fn check_parsed<T: Debug>(case: &ParseCase, tc: Timecode, source: T, source_type: &str) {
        assert_eq!(
            case.rate,
            tc.rate(),
            "tc rate for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.seconds,
            tc.seconds(),
            "rational seconds for {} source {:?}",
            source_type,
            source,
        );
        assert_eq!(
            case.frames,
            tc.frames(),
            "frames for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.timecode,
            tc.timecode(),
            "timecode for {} source {:?}",
            source_type,
            source,
        );
        assert_eq!(
            case.runtime,
            tc.runtime(9),
            "runtime for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.feet_and_frames,
            tc.feet_and_frames(),
            "feet and frames for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.premiere_ticks,
            tc.premiere_ticks(),
            "premiere ticks for {} source {:?}",
            source_type,
            source
        );
    }
}
