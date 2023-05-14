#[cfg(test)]
mod test {
    use num::{Rational64, ToPrimitive};
    use rstest::rstest;

    use crate::{
        rates, source_ppro_ticks::PremiereTicksSource, FeetFramesStr, FilmFormat, Framerate,
        FramesSource, Ntsc, SecondsSource, Timecode, TimecodeParseError,
    };
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
        feet_and_frames_35mm_4perf: String,
        feet_and_frames_35mm_3perf: String,
        feet_and_frames_35mm_2perf: String,
        feet_and_frames_16mm: String,
        premiere_ticks: i64,
    }

    #[rstest]
    // 23.98 NTSC CASES ----------
    // ---------------------------
    #[case::t01_00_00_00_f23_98(
        ParseCase{
            frames_sources: vec![
                Box::new("01:00:00:00".to_string()),
                Box::new("86400".to_string()),
                Box::new("5400+00".to_string()),
                Box::new("4050+00.0".to_string()),
                Box::new(FeetFramesStr::new("5400+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("4050+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("2700+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("4320+00", FilmFormat::FF16mm)),
                Box::new(86400i64),
                Box::new(86400u64),
                Box::new(86400i32),
                Box::new(86400u32),
                Box::new(86400isize),
                Box::new(86400usize),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(18018, 5)),
                Box::new(Rational64::new(18018, 5).to_f64().unwrap()),
                Box::new(Rational64::new(18018, 5).to_f32().unwrap()),
                Box::new("01:00:03.6".to_string()),
                Box::new("3603.6".to_string()),
            ],
            ticks_sources: vec![
                Box::new(915372057600000i64),
                Box::new(915372057600000u64),
            ],
            rate: rates::F23_98,
            seconds: Rational64::new(18018, 5),
            frames: 86400,
            timecode: "01:00:00:00".to_string(),
            runtime: "01:00:03.6".to_string(),
            feet_and_frames_35mm_4perf: "5400+00".to_string(),
            feet_and_frames_35mm_3perf: "4050+00.0".to_string(),
            feet_and_frames_35mm_2perf: "2700+00".to_string(),
            feet_and_frames_16mm: "4320+00".to_string(),         
            premiere_ticks: 915372057600000i64,
        }
    )]
    #[case::t01_00_00_00_f23_98_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-01:00:00:00".to_string()),
                Box::new("-86400".to_string()),
                Box::new("-5400+00".to_string()),
                Box::new("-4050+00.0".to_string()),
                Box::new(FeetFramesStr::new("-5400+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("-4050+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("-2700+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("-4320+00", FilmFormat::FF16mm)),             
                Box::new(-86400i64),
                Box::new(-86400i32),
                Box::new(-86400isize),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(18018, 5)),
                Box::new(-Rational64::new(18018, 5).to_f64().unwrap()),
                Box::new(-Rational64::new(18018, 5).to_f32().unwrap()),
                Box::new("-01:00:03.6".to_string()),
                Box::new("-3603.6".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-915372057600000i64),
            ],
            rate: rates::F23_98,
            seconds: -Rational64::new(18018, 5),
            frames: -86400,
            timecode: "-01:00:00:00".to_string(),
            runtime: "-01:00:03.6".to_string(),
            feet_and_frames_35mm_4perf: "-5400+00".to_string(),
            feet_and_frames_35mm_3perf: "-4050+00.0".to_string(),
            feet_and_frames_35mm_2perf: "-2700+00".to_string(),
            feet_and_frames_16mm: "-4320+00".to_string(),
            premiere_ticks: -915372057600000i64,
        }
    )]
    #[case::t00_40_00_00_f23_98(
        ParseCase{
            frames_sources: vec![
                Box::new("00:40:00:00".to_string()),
                Box::new("57600".to_string()),
                Box::new("3600+00".to_string()),
                Box::new("2700+00.0".to_string()),
                Box::new(FeetFramesStr::new("3600+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("2700+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("1800+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("2880+00", FilmFormat::FF16mm)),
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
            feet_and_frames_35mm_4perf: "3600+00".to_string(),
            feet_and_frames_35mm_3perf: "2700+00.0".to_string(),
            feet_and_frames_35mm_2perf: "1800+00".to_string(),
            feet_and_frames_16mm: "2880+00".to_string(),
            premiere_ticks: 610248038400000,
        }
    )]
    #[case::t00_40_00_00_f23_98_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:40:00:00".to_string()),
                Box::new("-57600".to_string()),
                Box::new("-3600+00".to_string()),
                Box::new("-2700+00.0".to_string()),
                Box::new(FeetFramesStr::new("-3600+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("-2700+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("-1800+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("-2880+00", FilmFormat::FF16mm)),              
                Box::new(-57600i64),
                Box::new(-57600i32),
                Box::new(-57600isize),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(12012, 5)),
                Box::new(-Rational64::new(12012, 5).to_f64().unwrap()),
                Box::new(-Rational64::new(12012, 5).to_f32().unwrap()),
                Box::new("-00:40:02.4".to_string()),
                Box::new("-2402.4".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-610248038400000i64),
            ],
            rate: rates::F23_98,
            seconds: -Rational64::new(12012, 5),
            frames: -57600,
            timecode: "-00:40:00:00".to_string(),
            runtime: "-00:40:02.4".to_string(),
            feet_and_frames_35mm_4perf: "-3600+00".to_string(),
            feet_and_frames_35mm_3perf: "-2700+00.0".to_string(),
            feet_and_frames_35mm_2perf: "-1800+00".to_string(),
            feet_and_frames_16mm: "-2880+00".to_string(),
            premiere_ticks: -610248038400000,
        }
    )]
    // 24 CASES ------------------
    // ---------------------------
    #[case::t01_00_00_00_f24(
        ParseCase{
            frames_sources: vec![
                Box::new("01:00:00:00".to_string()),
                Box::new("86400".to_string()),
                Box::new("5400+00".to_string()),
                Box::new("4050+00.0".to_string()),
                Box::new(FeetFramesStr::new("5400+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("4050+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("2700+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("4320+00", FilmFormat::FF16mm)),
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
            feet_and_frames_35mm_4perf: "5400+00".to_string(),
            feet_and_frames_35mm_3perf: "4050+00.0".to_string(),
            feet_and_frames_35mm_2perf: "2700+00".to_string(),
            feet_and_frames_16mm: "4320+00".to_string(),
            premiere_ticks: 914457600000000,
        }
    )]
    #[case::t01_00_00_00_f24_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-01:00:00:00".to_string()),
                Box::new("-86400".to_string()),
                Box::new("-5400+00".to_string()),
                Box::new("-4050+00.0".to_string()),
                Box::new(FeetFramesStr::new("-5400+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("-4050+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("-2700+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("-4320+00", FilmFormat::FF16mm)),
                Box::new(-86400i64),
                Box::new(-86400i32),
                Box::new(-86400isize),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(3600, 1)),
                Box::new(-Rational64::new(3600, 1).to_f64().unwrap()),
                Box::new(-Rational64::new(3600, 1).to_f32().unwrap()),
                Box::new("-01:00:00.0".to_string()),
                Box::new("-3600.0".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-914457600000000i64),
            ],
            rate: rates::F24,
            seconds: -Rational64::new(3600, 1),
            frames: -86400,
            timecode: "-01:00:00:00".to_string(),
            runtime: "-01:00:00.0".to_string(),
            feet_and_frames_35mm_4perf: "-5400+00".to_string(),
            feet_and_frames_35mm_3perf: "-4050+00.0".to_string(),
            feet_and_frames_35mm_2perf: "-2700+00".to_string(),
            feet_and_frames_16mm: "-4320+00".to_string(),
            premiere_ticks: -914457600000000,
        }
    )]
    #[case::t00_40_00_00_f24(
        ParseCase{
            frames_sources: vec![
                Box::new("00:40:00:00".to_string()),
                Box::new("57600".to_string()),
                Box::new("3600+00".to_string()),
                Box::new("2700+00.0".to_string()),
                Box::new(FeetFramesStr::new("3600+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("2700+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("1800+00", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("2880+00", FilmFormat::FF16mm)),
                Box::new(57600i64),
                Box::new(57600u64),
                Box::new(57600i32),
                Box::new(57600u32),
                Box::new(57600isize),
                Box::new(57600usize),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(2400, 1)),
                Box::new(Rational64::new(2400, 1).to_f64().unwrap()),
                Box::new(Rational64::new(2400, 1).to_f32().unwrap()),
                Box::new("00:40:00.0".to_string()),
                Box::new("2400.0".to_string()),
                Box::new("2400".to_string()),
            ],
            ticks_sources: vec![
                Box::new(609638400000000i64),
                Box::new(609638400000000u64),
            ],
            rate: rates::F24,
            seconds: Rational64::new(2400, 1),
            frames: 57600,
            timecode: "00:40:00:00".to_string(),
            runtime: "00:40:00.0".to_string(),
            feet_and_frames_35mm_4perf: "3600+00".to_string(),
            feet_and_frames_35mm_3perf: "2700+00.0".to_string(),
            feet_and_frames_35mm_2perf: "1800+00".to_string(),
            feet_and_frames_16mm: "2880+00".to_string(),
            premiere_ticks: 609638400000000,
        }
    )]
    #[case::t00_40_00_00_f24_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:40:00:00".to_string()),
                Box::new("-57600".to_string()),
                Box::new("-3600+00".to_string()),
                Box::new("-2700+00.0".to_string()),
                Box::new(FeetFramesStr::new("-3600+0", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("-2700+00.0", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("-1800+0", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("-2880+0", FilmFormat::FF16mm)),
                Box::new(-57600i64),
                Box::new(-57600i32),
                Box::new(-57600isize),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(2400, 1)),
                Box::new(-Rational64::new(2400, 1).to_f64().unwrap()),
                Box::new(-Rational64::new(2400, 1).to_f32().unwrap()),
                Box::new("-00:40:00.0".to_string()),
                Box::new("-2400.0".to_string()),
                Box::new("-2400".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-609638400000000i64),
            ],
            rate: rates::F24,
            seconds: -Rational64::new(2400, 1),
            frames: -57600,
            timecode: "-00:40:00:00".to_string(),
            runtime: "-00:40:00.0".to_string(),
            feet_and_frames_35mm_4perf: "-3600+00".to_string(),
            feet_and_frames_35mm_3perf: "-2700+00.0".to_string(),
            feet_and_frames_35mm_2perf: "-1800+00".to_string(),
            feet_and_frames_16mm: "-2880+00".to_string(),
            premiere_ticks: -609638400000000,
        }
    )]
    // 29.97 DF CASES ------------
    // ---------------------------
    #[case::t00_00_00_00_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:00:00;00".to_string()),
                Box::new("0+00".to_string()),
                Box::new("0+00.0".to_string()),
                Box::new(FeetFramesStr::new("0+00", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("0+0", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("0+00", FilmFormat::FF16mm)),
                Box::new(0i64),
                Box::new(0u64),
                Box::new(0i32),
                Box::new(0u32),
                Box::new(0i16),
                Box::new(0u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(0, 1)),
                Box::new(Rational64::new(0, 1).to_f64().unwrap()),
                Box::new(Rational64::new(0, 1).to_f32().unwrap()),
                Box::new("00:00:00.0".to_string()),
            ],
            ticks_sources: vec![
                Box::new(0i64),
                Box::new(0u64),
            ],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(0, 1),
            frames: 0,
            timecode: "00:00:00;00".to_string(),
            runtime: "00:00:00.0".to_string(),
            feet_and_frames_35mm_4perf: "0+00".to_string(),
            feet_and_frames_35mm_3perf: "0+00.0".to_string(),
            feet_and_frames_35mm_2perf: "0+00".to_string(),
            feet_and_frames_16mm: "0+00".to_string(),
            premiere_ticks: 0,
        }
    )]
    #[case::t00_01_01_00_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:01:01;00".to_string()),
                Box::new("1828".to_string()),
                Box::new("114+04".to_string()),
                Box::new("85+15.1".to_string()),
                Box::new(FeetFramesStr::new("114+04", FilmFormat::FF35mm4perf)),
                Box::new(FeetFramesStr::new("85+15.1", FilmFormat::FF35mm3perf)),
                Box::new(FeetFramesStr::new("57+4", FilmFormat::FF35mm2perf)),
                Box::new(FeetFramesStr::new("91+08", FilmFormat::FF16mm)),
                Box::new(1828i64),
                Box::new(1828u64),
                Box::new(1828i32),
                Box::new(1828u32),
                Box::new(1828i16),
                Box::new(1828u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(457_457, 7500)),
                Box::new(Rational64::new(457_457, 7500).to_f64().unwrap()),
                Box::new(Rational64::new(457_457, 7500).to_f32().unwrap()),
                Box::new("00:01:00.994266667".to_string()),
            ],
            ticks_sources: vec![],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(457_457, 7500),
            frames: 1828,
            timecode: "00:01:01;00".to_string(),
            runtime: "00:01:00.994266667".to_string(),
            feet_and_frames_35mm_4perf: "114+04".to_string(),
            feet_and_frames_35mm_3perf: "85+15.1".to_string(),
            feet_and_frames_35mm_2perf: "57+04".to_string(),
            feet_and_frames_16mm: "91+08".to_string(),
            premiere_ticks: 15_493_519_641_600,
        }
    )]
    #[case::t00_00_02_02_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:00:02;02".to_string()),
                Box::new("62".to_string()),
                Box::new("3+14".to_string()),
                Box::new("2+20.2".to_string()),
                Box::new(62i64),
                Box::new(62u64),
                Box::new(62i32),
                Box::new(62u32),
                Box::new(62i16),
                Box::new(62u16),
                Box::new(62i8),
                Box::new(62u8),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(31031, 15000)),
                Box::new(Rational64::new(31031, 15000).to_f64().unwrap()),
                Box::new(Rational64::new(31031, 15000).to_f32().unwrap()),
                Box::new("00:00:02.068733333".to_string()),
                Box::new("2.068733333".to_string()),
            ],
            ticks_sources: vec![
                Box::new(525491366400i64),
                Box::new(525491366400u64),
            ],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(31031, 15000),
            frames: 62,
            timecode: "00:00:02;02".to_string(),
            runtime: "00:00:02.068733333".to_string(),
            feet_and_frames_35mm_4perf: "3+14".to_string(),
            feet_and_frames_35mm_3perf: "2+20.2".to_string(),
            feet_and_frames_35mm_2perf: "1+30".to_string(),
            feet_and_frames_16mm: "3+02".to_string(),
            premiere_ticks: 525491366400,
        }
    )]
    #[case::t00_00_02_02_f29_97_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:00:02;02".to_string()),
                Box::new("-62".to_string()),
                Box::new("-3+14".to_string()),
                Box::new("-2+20.2".to_string()),
                Box::new(-62i64),
                Box::new(-62i32),
                Box::new(-62i16),
                Box::new(-62i8),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(31031, 15000)),
                Box::new(-Rational64::new(31031, 15000).to_f64().unwrap()),
                Box::new(-Rational64::new(31031, 15000).to_f32().unwrap()),
                Box::new("-00:00:02.068733333".to_string()),
                Box::new("-2.068733333".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-525491366400i64),
            ],
            rate: rates::F29_97_DF,
            seconds: -Rational64::new(31031, 15000),
            frames: -62,
            timecode: "-00:00:02;02".to_string(),
            runtime: "-00:00:02.068733333".to_string(),
            feet_and_frames_35mm_4perf: "-3+14".to_string(),
            feet_and_frames_35mm_3perf: "-2+20.2".to_string(),
            feet_and_frames_35mm_2perf: "-1+30".to_string(),
            feet_and_frames_16mm: "-3+02".to_string(),
            premiere_ticks: -525491366400,
        }
    )]
    #[case::t00_01_00_02_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:01:00;02".to_string()),
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
            feet_and_frames_35mm_4perf: "112+08".to_string(),
            feet_and_frames_35mm_3perf: "84+08.0".to_string(),
            feet_and_frames_35mm_2perf: "56+08".to_string(),
            feet_and_frames_16mm: "90+00".to_string(),
            premiere_ticks: 15256200960000,
        }
    )]
    #[case::t00_00_01_02_f29_97_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:01:00;02".to_string()),
                Box::new("-1800".to_string()),
                Box::new("-112+08".to_string()),
                Box::new(-1800i64),
                Box::new(-1800i32),
                Box::new(-1800i16),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(3003, 50)),
                Box::new(-Rational64::new(3003, 50).to_f64().unwrap()),
                Box::new(-Rational64::new(3003, 50).to_f32().unwrap()),
                Box::new("-00:01:00.06".to_string()),
                Box::new("-60.06".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-15256200960000i64),
            ],
            rate: rates::F29_97_DF,
            seconds: -Rational64::new(3003, 50),
            frames: -1800,
            timecode: "-00:01:00;02".to_string(),
            runtime: "-00:01:00.06".to_string(),
            feet_and_frames_35mm_4perf: "-112+08".to_string(),
            feet_and_frames_35mm_3perf: "-84+08.0".to_string(),
            feet_and_frames_35mm_2perf: "-56+08".to_string(),
            feet_and_frames_16mm: "-90+00".to_string(),
            premiere_ticks: -15256200960000,
        }
    )]
    #[case::t00_10_00_00_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:10:00;00".to_string()),
                Box::new("17982".to_string()),
                Box::new("1123+14".to_string()),
                Box::new("842+20.2".to_string()),
                Box::new(17982i64),
                Box::new(17982u64),
                Box::new(17982i32),
                Box::new(17982u32),
                Box::new(17982i16),
                Box::new(17982u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(2999997, 5000)),
                Box::new(Rational64::new(2999997, 5000).to_f64().unwrap()),
                Box::new(Rational64::new(2999997, 5000).to_f32().unwrap()),
                Box::new("00:09:59.9994".to_string()),
                Box::new("599.9994".to_string()),
            ],
            ticks_sources: vec![
                Box::new(152409447590400i64),
                Box::new(152409447590400u64),
            ],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(2999997, 5000),
            frames: 17982,
            timecode: "00:10:00;00".to_string(),
            runtime: "00:09:59.9994".to_string(),
            feet_and_frames_35mm_4perf: "1123+14".to_string(),
            feet_and_frames_35mm_3perf: "842+20.2".to_string(),
            feet_and_frames_35mm_2perf: "561+30".to_string(),
            feet_and_frames_16mm: "899+02".to_string(),
            premiere_ticks: 152409447590400,
        }
    )]
    #[case::t00_10_00_00_f29_97_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:10:00;00".to_string()),
                Box::new("-17982".to_string()),
                Box::new("-1123+14".to_string()),
                Box::new("-842+20.2".to_string()),
                Box::new(-17982i64),
                Box::new(-17982i32),
                Box::new(-17982i16),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(2999997, 5000)),
                Box::new(-Rational64::new(2999997, 5000).to_f64().unwrap()),
                Box::new(-Rational64::new(2999997, 5000).to_f32().unwrap()),
                Box::new("-00:09:59.9994".to_string()),
                Box::new("-599.9994".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-152409447590400i64),
            ],
            rate: rates::F29_97_DF,
            seconds: -Rational64::new(2999997, 5000),
            frames: -17982,
            timecode: "-00:10:00;00".to_string(),
            runtime: "-00:09:59.9994".to_string(),
            feet_and_frames_35mm_4perf: "-1123+14".to_string(),
            feet_and_frames_35mm_3perf: "-842+20.2".to_string(),
            feet_and_frames_35mm_2perf: "-561+30".to_string(),
            feet_and_frames_16mm: "-899+02".to_string(),
            premiere_ticks: -152409447590400,
        }
    )]
    #[case::t00_11_00_02_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:11:00;02".to_string()),
                Box::new("19782".to_string()),
                Box::new("1236+06".to_string()),
                Box::new(19782i64),
                Box::new(19782u64),
                Box::new(19782i32),
                Box::new(19782u32),
                Box::new(19782i16),
                Box::new(19782u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(3300297, 5000)),
                Box::new(Rational64::new(3300297, 5000).to_f64().unwrap()),
                Box::new(Rational64::new(3300297, 5000).to_f32().unwrap()),
                Box::new("00:11:00.0594".to_string()),
                Box::new("660.0594".to_string()),
            ],
            ticks_sources: vec![
                Box::new(167665648550400i64),
                Box::new(167665648550400u64),
            ],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(3300297, 5000),
            frames: 19782,
            timecode: "00:11:00;02".to_string(),
            runtime: "00:11:00.0594".to_string(),
            feet_and_frames_35mm_4perf: "1236+06".to_string(),
            feet_and_frames_35mm_3perf: "927+06.0".to_string(),
            feet_and_frames_35mm_2perf: "618+06".to_string(), 
            feet_and_frames_16mm: "989+02".to_string(), 
            premiere_ticks: 167665648550400,
        }
    )]
    #[case::t00_11_00_02_f29_97_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:11:00;02".to_string()),
                Box::new("-19782".to_string()),
                Box::new("-1236+06".to_string()),
                Box::new(-19782i64),
                Box::new(-19782i32),
                Box::new(-19782i16),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(3300297, 5000)),
                Box::new(-Rational64::new(3300297, 5000).to_f64().unwrap()),
                Box::new(-Rational64::new(3300297, 5000).to_f32().unwrap()),
                Box::new("-00:11:00.0594".to_string()),
                Box::new("-660.0594".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-167665648550400i64),
            ],
            rate: rates::F29_97_DF,
            seconds: -Rational64::new(3300297, 5000),
            frames: -19782,
            timecode: "-00:11:00;02".to_string(),
            runtime: "-00:11:00.0594".to_string(),
            feet_and_frames_35mm_4perf: "-1236+06".to_string(),
            feet_and_frames_35mm_3perf: "-927+06.0".to_string(), 
            feet_and_frames_35mm_2perf: "-618+06".to_string(), 
            feet_and_frames_16mm: "-989+02".to_string(), 
            premiere_ticks: -167665648550400,
        }
    )]
    #[case::t01_00_00_00_f29_97_df(
        ParseCase{
            frames_sources: vec![
                Box::new("01:00:00;00".to_string()),
                Box::new("107892".to_string()),
                Box::new("6743+04".to_string()),
                Box::new("5057+10.2".to_string()),
                Box::new(107892i64),
                Box::new(107892u64),
                Box::new(107892i32),
                Box::new(107892u32),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(8999991, 2500)),
                Box::new(Rational64::new(8999991, 2500).to_f64().unwrap()),
                Box::new(Rational64::new(8999991, 2500).to_f32().unwrap()),
                Box::new("00:59:59.9964".to_string()),
                Box::new("3599.9964".to_string()),
            ],
            ticks_sources: vec![
                Box::new(914456685542400i64),
                Box::new(914456685542400u64),
            ],
            rate: rates::F29_97_DF,
            seconds: Rational64::new(8999991, 2500),
            frames: 107892,
            timecode: "01:00:00;00".to_string(),
            runtime: "00:59:59.9964".to_string(),
            feet_and_frames_35mm_4perf: "6743+04".to_string(),
            feet_and_frames_35mm_3perf: "5057+10.2".to_string(), 
            feet_and_frames_35mm_2perf: "3371+20".to_string(), 
            feet_and_frames_16mm: "5394+12".to_string(), 
            premiere_ticks: 914456685542400,
        }
    )]
    #[case::t01_00_00_00_f29_97_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-01:00:00;00".to_string()),
                Box::new("-107892".to_string()),
                Box::new("-6743+04".to_string()),
                Box::new("-5057+10.2".to_string()),
                Box::new(-107892i64),
                Box::new(-107892i32),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(8999991, 2500)),
                Box::new(-Rational64::new(8999991, 2500).to_f64().unwrap()),
                Box::new(-Rational64::new(8999991, 2500).to_f32().unwrap()),
                Box::new("-00:59:59.9964".to_string()),
                Box::new("-3599.9964".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-914456685542400i64),
            ],
            rate: rates::F29_97_DF,
            seconds: -Rational64::new(8999991, 2500),
            frames: -107892,
            timecode: "-01:00:00;00".to_string(),
            runtime: "-00:59:59.9964".to_string(),
            feet_and_frames_35mm_4perf: "-6743+04".to_string(),
            feet_and_frames_35mm_3perf: "-5057+10.2".to_string(), 
            feet_and_frames_35mm_2perf: "-3371+20".to_string(), 
            feet_and_frames_16mm: "-5394+12".to_string(), 
            premiere_ticks: -914456685542400,
        }
    )]
    // 59.94 DF CASES ------------
    // ---------------------------
    #[case::t00_00_00_00_f59_94_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:00:00;00".to_string()),
                Box::new("0+00".to_string()),
                Box::new(0i64),
                Box::new(0u64),
                Box::new(0i32),
                Box::new(0u32),
                Box::new(0i16),
                Box::new(0u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(0, 1)),
                Box::new(Rational64::new(0, 1).to_f64().unwrap()),
                Box::new(Rational64::new(0, 1).to_f32().unwrap()),
                Box::new("0".to_string()),
            ],
            ticks_sources: vec![
                Box::new(0i64),
                Box::new(0u64),
            ],
            rate: rates::F59_94_DF,
            seconds: Rational64::new(0, 1),
            frames: 0,
            timecode: "00:00:00;00".to_string(),
            runtime: "00:00:00.0".to_string(),
            feet_and_frames_35mm_4perf: "0+00".to_string(),
            feet_and_frames_35mm_3perf: "0+00.0".to_string(), 
            feet_and_frames_35mm_2perf: "0+00".to_string(), 
            feet_and_frames_16mm: "0+00".to_string(), 
            premiere_ticks: 0,
        }
    )]
    #[case::t00_00_01_01_f59_94_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:00:01;01".to_string()),
                Box::new("61".to_string()),
                Box::new("3+13".to_string()),
                Box::new("2+19.2".to_string()),
                Box::new(61i64),
                Box::new(61u64),
                Box::new(61i32),
                Box::new(61u32),
                Box::new(61i16),
                Box::new(61u16),
                Box::new(61i8),
                Box::new(61u8),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(61061, 60000)),
                Box::new(Rational64::new(61061, 60000).to_f64().unwrap()),
                Box::new(Rational64::new(61061, 60000).to_f32().unwrap()),
                Box::new("00:00:01.017683333".to_string()),
                Box::new("1.017683333333333333333333333".to_string()),
            ],
            ticks_sources: vec![
                Box::new(258507849600i64),
                Box::new(258507849600u64),
            ],
            rate: rates::F59_94_DF,
            seconds: Rational64::new(61061, 60000),
            frames: 61,
            timecode: "00:00:01;01".to_string(),
            runtime: "00:00:01.017683333".to_string(),
            feet_and_frames_35mm_4perf: "3+13".to_string(),
            feet_and_frames_35mm_3perf: "2+19.2".to_string(),
            feet_and_frames_35mm_2perf: "1+29".to_string(), 
            feet_and_frames_16mm: "3+01".to_string(), 
            premiere_ticks: 258507849600,
        }
    )]
    #[case::t00_00_01_01_f59_94_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:00:01;01".to_string()),
                Box::new("-61".to_string()),
                Box::new("-3+13".to_string()),
                Box::new("-2+19.2".to_string()),
                Box::new(-61i64),
                Box::new(-61i32),
                Box::new(-61i16),
                Box::new(-61i8),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(61061, 60000)),
                Box::new(-Rational64::new(61061, 60000).to_f64().unwrap()),
                Box::new(-Rational64::new(61061, 60000).to_f32().unwrap()),
                Box::new("-00:00:01.017683333".to_string()),
                Box::new("-1.017683333333333333333333333".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-258507849600i64),
            ],
            rate: rates::F59_94_DF,
            seconds: -Rational64::new(61061, 60000),
            frames: -61,
            timecode: "-00:00:01;01".to_string(),
            runtime: "-00:00:01.017683333".to_string(),
            feet_and_frames_35mm_4perf: "-3+13".to_string(),
            feet_and_frames_35mm_3perf: "-2+19.2".to_string(), 
            feet_and_frames_35mm_2perf: "-1+29".to_string(), 
            feet_and_frames_16mm: "-3+01".to_string(), 
            premiere_ticks: -258507849600,
        }
    )]
    #[case::t00_00_01_03_f59_94_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:00:01;03".to_string()),
                Box::new("63".to_string()),
                Box::new("3+15".to_string()),
                Box::new("2+21.2".to_string()),
                Box::new(63i64),
                Box::new(63u64),
                Box::new(63i32),
                Box::new(63u32),
                Box::new(63i16),
                Box::new(63u16),
                Box::new(63i8),
                Box::new(63u8),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(21021, 20000)),
                Box::new(Rational64::new(21021, 20000).to_f64().unwrap()),
                Box::new(Rational64::new(21021, 20000).to_f32().unwrap()),
                Box::new("00:00:01.05105".to_string()),
                Box::new("1.05105".to_string()),
            ],
            ticks_sources: vec![
                Box::new(266983516800i64),
                Box::new(266983516800u64),
            ],
            rate: rates::F59_94_DF,
            seconds: Rational64::new(21021, 20000),
            frames: 63,
            timecode: "00:00:01;03".to_string(),
            runtime: "00:00:01.05105".to_string(),
            feet_and_frames_35mm_4perf: "3+15".to_string(),
            feet_and_frames_35mm_3perf: "2+21.2".to_string(), 
            feet_and_frames_35mm_2perf: "1+31".to_string(), 
            feet_and_frames_16mm: "3+03".to_string(), 
            premiere_ticks: 266983516800,
        }
    )]
    #[case::t00_00_01_03_f59_94_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:00:01;03".to_string()),
                Box::new("-63".to_string()),
                Box::new("-3+15".to_string()),
                Box::new("-2+21.2".to_string()),
                Box::new(-63i64),
                Box::new(-63i32),
                Box::new(-63i16),
                Box::new(-63i8),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(21021, 20000)),
                Box::new(-Rational64::new(21021, 20000).to_f64().unwrap()),
                Box::new(-Rational64::new(21021, 20000).to_f32().unwrap()),
                Box::new("-00:00:01.05105".to_string()),
                Box::new("-1.05105".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-266983516800i64),
            ],
            rate: rates::F59_94_DF,
            seconds: -Rational64::new(21021, 20000),
            frames: -63,
            timecode: "-00:00:01;03".to_string(),
            runtime: "-00:00:01.05105".to_string(),
            feet_and_frames_35mm_4perf: "-3+15".to_string(),
            feet_and_frames_35mm_3perf: "-2+21.2".to_string(), 
            feet_and_frames_35mm_2perf: "-1+31".to_string(), 
            feet_and_frames_16mm: "-3+03".to_string(), 
            premiere_ticks: -266983516800,
        }
    )]
    // This is the first minute we should be skipping frames on. For 59.94 we
    // skip 4 frames.
    #[case::t00_01_00_04_f59_94_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:01:00;04".to_string()),
                Box::new("3600".to_string()),
                Box::new("225+00".to_string()),
                Box::new(3600i64),
                Box::new(3600u64),
                Box::new(3600i32),
                Box::new(3600u32),
                Box::new(3600i16),
                Box::new(3600u16),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(3003, 50)),
                Box::new(Rational64::new(3003, 50).to_f64().unwrap()),
                Box::new(Rational64::new(3003, 50).to_f32().unwrap()),
                Box::new("00:01:00.06".to_string()),
                Box::new("60.06".to_string()),
            ],
            ticks_sources: vec![
                Box::new(15256200960000i64),
                Box::new(15256200960000u64),
            ],
            rate: rates::F59_94_DF,
            seconds: Rational64::new(3003, 50),
            frames: 3600,
            timecode: "00:01:00;04".to_string(),
            runtime: "00:01:00.06".to_string(),
            feet_and_frames_35mm_4perf: "225+00".to_string(),
            feet_and_frames_35mm_3perf: "168+16.0".to_string(), 
            feet_and_frames_35mm_2perf: "112+16".to_string(), 
            feet_and_frames_16mm: "180+00".to_string(), 
            premiere_ticks: 15256200960000,
        }
    )]
    // Less than 4 frames past a minute should parse OK
    #[case::t00_01_00_04_f59_94_df(
        ParseCase{
            frames_sources: vec![
                Box::new("00:01:01;03".to_string()),
            ],
            seconds_sources: vec![],
            ticks_sources: vec![],
            rate: rates::F59_94_DF,
            seconds: Rational64::new(3662659, 60000),
            frames: 3659,
            timecode: "00:01:01;03".to_string(),
            runtime: "00:01:01.044316667".to_string(),
            feet_and_frames_35mm_4perf: "228+11".to_string(),
            feet_and_frames_35mm_3perf: "171+11.0".to_string(), 
            feet_and_frames_35mm_2perf: "114+11".to_string(), 
            feet_and_frames_16mm: "182+19".to_string(), 
            premiere_ticks: 15506233142400,
        }
    )]
    #[case::t00_01_00_04_f59_94_df_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("-00:01:00;04".to_string()),
                Box::new("-3600".to_string()),
                Box::new("-225+00".to_string()),
                Box::new(-3600i64),
                Box::new(-3600i32),
                Box::new(-3600i16),
            ],
            seconds_sources: vec![
                Box::new(-Rational64::new(3003, 50)),
                Box::new(-Rational64::new(3003, 50).to_f64().unwrap()),
                Box::new(-Rational64::new(3003, 50).to_f32().unwrap()),
                Box::new("-00:01:00.06".to_string()),
                Box::new("-60.06".to_string()),
            ],
            ticks_sources: vec![
                Box::new(-15256200960000i64),
            ],
            rate: rates::F59_94_DF,
            seconds: -Rational64::new(3003, 50),
            frames: -3600,
            timecode: "-00:01:00;04".to_string(),
            runtime: "-00:01:00.06".to_string(),
            feet_and_frames_35mm_4perf: "-225+00".to_string(),
            feet_and_frames_35mm_3perf: "-168+16.0".to_string(), 
            feet_and_frames_35mm_2perf: "-112+16".to_string(), 
            feet_and_frames_16mm: "-180+00".to_string(), 
            premiere_ticks: -15256200960000,
        }
    )]
    // 239.76 NDF CASES ---------------------
    // We're going to use this to test very large values beyond what you would normally see in the
    // wild to put pressure on possible integer overflow points.
    //
    // This value represetns a timecode of over 123 hours rrunning at 240 fps. In the real world,
    // one would be VERY unlikely to see a timecode like this. We are using an NTSC timebase as
    // NTSC bases are far more likely to create large numerators / denominators.
    #[case::t123_17_34_217_f239_76_ndf_negative(
        ParseCase{
            frames_sources: vec![
                Box::new("123:17:34:217".to_string()),
                Box::new("106525177".to_string()),
                Box::new(106525177i64),
                Box::new(106525177u64),
            ],
            seconds_sources: vec![
                Box::new(Rational64::new(106631702177, 240000)),
                Box::new(Rational64::new(106631702177, 240000).to_f64().unwrap()),
                // We are not going to run the f32 version of this test. The value is too imprecice 
                // to give us the correct answer at values this high.
                // Box::new(Rational64::new(106631702177, 240000).to_f32().unwrap()),
                Box::new("123:24:58.759070833".to_string()),
            ],
            ticks_sources: vec![
                Box::new(112858993584136800i64),
                Box::new(112858993584136800u64),
            ],
            rate: Framerate::with_playback(239.76, Ntsc::NonDropFrame).unwrap(),
            seconds: Rational64::new(106631702177, 240000),
            frames: 106525177,
            timecode: "123:17:34:217".to_string(),
            runtime: "123:24:58.759070833".to_string(),
            feet_and_frames_35mm_4perf: "6657823+09".to_string(),
            feet_and_frames_35mm_3perf: "4993367+15.2".to_string(), 
            feet_and_frames_35mm_2perf: "3328911+25".to_string(), 
            feet_and_frames_16mm: "5326258+17".to_string(), 
            premiere_ticks: 112858993584136800,
        }
    )]
    fn test_parse_timecode(#[case] case: ParseCase) -> Result<(), TimecodeParseError> {
        for boxed in case.frames_sources.iter() {
            let source = boxed.deref();
            let tc = Timecode::with_frames(source, case.rate)?;
            check_parsed(&case, tc, source, "frames")
        }

        for boxed in case.seconds_sources.iter() {
            let source = boxed.deref();
            let tc = Timecode::with_seconds(source, case.rate)?;
            check_parsed(&case, tc, source, "seconds")
        }

        for boxed in case.ticks_sources.iter() {
            let source = boxed.deref();
            let tc = Timecode::with_premiere_ticks(source, case.rate)?;
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
            case.feet_and_frames_35mm_4perf,
            tc.feet_and_frames(FilmFormat::FF35mm4perf),
            "feet and frames (35mm 4p) for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.feet_and_frames_35mm_3perf,
            tc.feet_and_frames(FilmFormat::FF35mm3perf),
            "feet and frames (35mm 3p) for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.feet_and_frames_35mm_2perf,
            tc.feet_and_frames(FilmFormat::FF35mm2perf),
            "feet and frames (35mm 2p) for {} source {:?}",
            source_type,
            source
        );
        assert_eq!(
            case.feet_and_frames_16mm,
            tc.feet_and_frames(FilmFormat::FF16mm),
            "feet and frames (16mm )for {} source {:?}",
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

    struct MalformedCase {
        tc_in: String,
        tc_out: String,
    }

    /// tests that timecodes with overflowed values (like "01:00:60:00) are parsed correctly.
    ///
    /// All cases assume 24 fps.
    #[rstest]
    #[case(MalformedCase{
        tc_in: "00:59:59:24".to_string(),
        tc_out: "01:00:00:00".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "00:59:59:28".to_string(),
        tc_out: "01:00:00:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "00:00:62:04".to_string(),
        tc_out: "00:01:02:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "00:62:01:04".to_string(),
        tc_out: "01:02:01:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "00:62:62:04".to_string(),
        tc_out: "01:03:02:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "123:00:00:00".to_string(),
        tc_out: "123:00:00:00".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "01:00:00:48".to_string(),
        tc_out: "01:00:02:00".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "01:00:120:00".to_string(),
        tc_out: "01:02:00:00".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "01:120:00:00".to_string(),
        tc_out: "03:00:00:00".to_string(),
    })]
    fn test_parse_overflows(#[case] case: MalformedCase) -> Result<(), TimecodeParseError> {
        let tc = Timecode::with_frames(case.tc_in, rates::F24)?;

        assert_eq!(case.tc_out, tc.timecode(), "parsed tc correct");

        Ok(())
    }

    /// tests that bad drop frame values fail to parse
    #[rstest]
    #[case("00:09:00:01", rates::F29_97_DF)]
    #[case("00:08:00:01", rates::F59_94_DF)]
    #[case("00:08:00:02", rates::F59_94_DF)]
    #[case("00:08:00:03", rates::F59_94_DF)]
    #[case("01:09:00:01", rates::F29_97_DF)]
    #[case("02:08:00:01", rates::F59_94_DF)]
    #[case("03:08:00:02", rates::F59_94_DF)]
    #[case("26:08:00:03", rates::F59_94_DF)]
    fn test_parse_bad_drop_frame(
        #[case] tc_str: &str,
        #[case] rate: Framerate,
    ) -> Result<(), TimecodeParseError> {
        let tc = Timecode::with_frames(tc_str, rate);
        assert!(tc.is_err());

        Ok(())
    }

    /// tests that timecode missing sections or digits is parsed correctly.
    #[rstest]
    #[case(MalformedCase{
        tc_in: "1:02:03:04".to_string(),
        tc_out: "01:02:03:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "02:03:04".to_string(),
        tc_out: "00:02:03:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "2:03:04".to_string(),
        tc_out: "00:02:03:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "03:04".to_string(),
        tc_out: "00:00:03:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "3:04".to_string(),
        tc_out: "00:00:03:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "04".to_string(),
        tc_out: "00:00:00:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "4".to_string(),
        tc_out: "00:00:00:04".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "1:2:3:4".to_string(),
        tc_out: "01:02:03:04".to_string(),
    })]
    fn test_parse_partial_tc(#[case] case: MalformedCase) -> Result<(), TimecodeParseError> {
        let tc = Timecode::with_frames(case.tc_in, rates::F24)?;

        assert_eq!(case.tc_out, tc.timecode(), "parsed tc correct");

        Ok(())
    }

    #[rstest]
    #[case(MalformedCase{
        tc_in: "1:02:03.5".to_string(),
        tc_out: "01:02:03:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "02:03.5".to_string(),
        tc_out: "00:02:03:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "2:03.5".to_string(),
        tc_out: "00:02:03:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "03.5".to_string(),
        tc_out: "00:00:03:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "03.5".to_string(),
        tc_out: "00:00:03:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "3.5".to_string(),
        tc_out: "00:00:03:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "0.5".to_string(),
        tc_out: "00:00:00:12".to_string(),
    })]
    #[case(MalformedCase{
        tc_in: "1:2:3.5".to_string(),
        tc_out: "01:02:03:12".to_string(),
    })]
    fn test_parse_partial_runtime(#[case] case: MalformedCase) -> Result<(), TimecodeParseError> {
        let tc = Timecode::with_seconds(case.tc_in, rates::F24)?;

        assert_eq!(case.tc_out, tc.timecode(), "parsed tc correct");

        Ok(())
    }

    /// test footage dimensions
    #[rstest]
    #[case(FilmFormat::FF35mm4perf, 64, 16, 1)]
    #[case(FilmFormat::FF35mm3perf, 192, 64, 3)]
    #[case(FilmFormat::FF35mm2perf, 64, 32, 1)]
    #[case(FilmFormat::FF16mm, 20, 20, 1)]
    fn test_footage_dims(
        #[case] format: FilmFormat,
        #[case] perfs: i64,
        #[case] frames: i64,
        #[case] feet: i64,
    ) -> () {
        assert_eq!(format.footage_modulus_perf_count(), perfs);
        assert_eq!(format.footage_modulus_frame_count(), frames);
        assert_eq!(format.footage_modulus_footage_count(), feet);
    }

    /// test 35mm3perf film parsing
    #[rstest]
    #[case("0+0.0", 0)]
    #[case("1+0.1", 21)]
    #[case("2+0.2", 42)]
    #[case("3+0.0", 64)]
    fn test_threeperf_parsing(
        #[case] ff: &str,
        #[case] frame_count: i64,
    ) -> Result<(), TimecodeParseError> {
        let tc =
            Timecode::with_frames(FeetFramesStr::new(ff, FilmFormat::FF35mm3perf), rates::F24)?;
        assert_eq!(tc.frames(), frame_count);

        Ok(())
    }

    /// test 35mm3perf string generation
    #[rstest]
    #[case(0, "0+00.0")]
    #[case(20,"0+20.0")]
    #[case(21,"1+00.1")]
    #[case(41,"1+20.1")]
    #[case(42,"2+00.2")]
    fn test_threeperf_generation(
        #[case] frame_count: i64,
        #[case] ff: &str
        ) -> Result<(),TimecodeParseError> {
        let tc = Timecode::with_frames(frame_count, rates::F24)?;
        assert_eq!(tc.feet_and_frames(FilmFormat::FF35mm3perf) , ff);

        Ok(())
    }
}
