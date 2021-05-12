#[cfg(test)]
mod test {
    use crate::{Framerate, FramerateParseError, FramerateSource, Ntsc};
    use rstest::rstest;

    #[rstest]
    #[case(Ntsc::False, false)]
    #[case(Ntsc::NonDropFrame, true)]
    #[case(Ntsc::DropFrame, true)]
    fn test_ntsc(#[case] value: Ntsc, #[case] is_ntsc: bool) {
        assert_eq!(
            is_ntsc,
            value.is_ntsc(),
            "check() is expected for {:?}",
            value
        );
    }

    /// ParseCase is a Parsing test case.
    struct ParseCase<T: FramerateSource> {
        /// source is the source framerate value to parse frome.
        source: T,
        /// source_type is whether the source if a framerate or timebase.
        source_type: SourceType,
        /// ntsc is whether the source should be parsed as an NTSC framerate.
        ntsc: Ntsc,
        /// expected is the expected result. Use Err([message])
        expected: Result<Success, FramerateParseError>,
    }

    enum SourceType {
        Playback,
        Timebase,
    }

    struct Success {
        playback: num::Rational64,
        timebase: num::Rational64,
    }

    #[rstest]
    // NTSC TEST CASES ---------
    // -------------------------
    // NTSC From non-whole float
    #[case::float_2398_ntsc_playback(ParseCase{
        source: 23.98,
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From whole float rounded
    #[case::float_2398_ntsc_timebase(ParseCase{
        source: 24.0,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From non-whole float string
    #[case::strfloat_2398_ntsc_plaback(ParseCase{
        source: "23.98".to_string(),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From float string
    #[case::strfloat_2398_ntsc_base(ParseCase{
        source: "24.0".to_string(),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From int.
    #[case::int_2398_ntsc_timebase(ParseCase{
        source: 24,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From rational non-whole.
    #[case::rational_2398_ntsc_playback(ParseCase{
        source: num::Rational64::new(24000, 1001),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From rational whole number.
    #[case::rational_2398_ntsc_timebase(ParseCase{
        source: num::Rational64::new(24, 1),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NON-NTSC TEST CASES -----
    // -------------------------
    // From int.
    #[case::int_24_timebase(ParseCase{
        source: 24,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::int_24_playback(ParseCase{
        source: 24,
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // From rational non-whole.
    #[case::rational_2398_playback(ParseCase{
        source: num::Rational64::new(24000, 1001),
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24000, 1001),
        }),
    })]
    // From rational non-whole.
    #[case::rational_2398_timebase(ParseCase{
        source: num::Rational64::new(24000, 1001),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24000, 1001),
        }),
    })]
    // From rational whole number.
    #[case::rational_2398_timebase(ParseCase{
        source: num::Rational64::new(24, 1),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })] // From rational whole number.
    #[case::rational_2398_playback(ParseCase{
        source: num::Rational64::new(24, 1),
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // DROP FRAME CASES --------------
    // -------------------------------
    // Drop-frame 29.97 float - non-whole.
    #[case::float_2997_drop_playback(ParseCase{
        source: 29.97,
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 float - whole.
    #[case::float_2997_drop_timebase(ParseCase{
        source: 30.0,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 int.
    #[case::int_2997_drop_timebase(ParseCase{
        source: 30,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 rational - non-whole.
    #[case::rational_2997_drop_playback(ParseCase{
        source: num::Rational64::new(30000, 1001),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 rational - whole.
    #[case::rational_2997_drop_timebase(ParseCase{
        source: num::Rational64::new(30, 1),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string float non-whole.
    #[case::strfloat_2997_drop_playback(ParseCase{
        source: "29.97".to_string(),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string float whole.
    #[case::strfloat_2997_drop_timebase(ParseCase{
        source: "30.0".to_string(),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string int.
    #[case::strint_2997_drop_timebase(ParseCase{
        source: "30".to_string(),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string rational non-whole.
    #[case::strrational_2997_drop_playback(ParseCase{
        source: "30000/1001".to_string(),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string rational whole.
    #[case::strrational_2997_drop_timebase(ParseCase{
        source: "30/1".to_string(),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 59.94 float non-whole.
    #[case::float_5994_drop_playback(ParseCase{
        source: 59.94,
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(60000, 1001),
            timebase: num::Rational64::new(60, 1),
        }),
    })]
    // Drop-frame 59.94 int
    #[case::float_5994_drop_timebase(ParseCase{
        source: 60,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(60000, 1001),
            timebase: num::Rational64::new(60, 1),
        }),
    })]
    // OTHER TYPES ---------
    // ---------------------
    #[case::from_i64(ParseCase{
        source: 24i64,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_u64(ParseCase{
        source: 24u64,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_i32(ParseCase{
        source: 24i32,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_u32(ParseCase{
        source: 24u32,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_u16(ParseCase{
        source: 24u16,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_i16(ParseCase{
        source: 24i16,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_i8(ParseCase{
        source: 24i8,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_u8(ParseCase{
        source: 24u8,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_f32(ParseCase{
        source: 24f32,
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::from_rational32(ParseCase{
        source: num::Rational32::new(24, 1),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // ERROR CASES ---------
    // ---------------------
    #[case::error_ntsc_playback_bad_denom(ParseCase{
        source: num::Rational64::new(24, 1),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Err(FramerateParseError::Ntsc("ntsc framerates must be n/1001".to_string())),
    })]
    #[case::error_ntsc_timebase_bad_denom(ParseCase{
        source: "24000/1001",
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Err(FramerateParseError::Ntsc("ntsc timebases must be whole numbers".to_string())),
    })]
    #[case::error_drop_frame_bad_value(ParseCase{
        source: "24000/1001",
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Err(FramerateParseError::DropFrame("dropframe must have playback divisible by 30000/1001 (multiple of 29.97)".to_string())),
    })]
    #[case::error_drop_frame_bad_value(ParseCase{
        source: "24/1",
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Err(FramerateParseError::DropFrame("dropframe must have timebase divisible by 30 (multiple of 29.97)".to_string())),
    })]
    #[case::error_negative(ParseCase{
        source: -24,
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Err(FramerateParseError::Negative("framerates cannot be negative".to_string())),
    })]
    #[case::error_f64_nonntsc(ParseCase{
        source: 23.98f64,
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Err(FramerateParseError::Imprecise("float values cannot be parsed for non-NTSC Framerates due to imprecision".to_string())),
    })]
    #[case::error_f32_nonntsc(ParseCase{
        source: 23.98f32,
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Err(FramerateParseError::Imprecise("float values cannot be parsed for non-NTSC Framerates due to imprecision".to_string())),
    })]
    #[case::error_u64_overlfow(ParseCase{
        source: u64::MAX,
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Err(FramerateParseError::Conversion("error converting u64 to i64 : out of range integral type conversion attempted".to_string())),
    })]
    fn test_parse_framerate<T: FramerateSource>(#[case] case: ParseCase<T>) {
        let result = match case.source_type {
            SourceType::Playback => Framerate::new_with_playback(case.source, case.ntsc),
            SourceType::Timebase => Framerate::new_with_timebase(case.source, case.ntsc),
        };

        if case.expected.is_err() {
            assert_eq!(
                result.err(),
                case.expected.err(),
                "error messages are equal"
            );
            return;
        }

        let parsed = result.unwrap();
        let expected = case.expected.unwrap();
        assert_eq!(parsed.playback(), expected.playback, "playback is expected");
        assert_eq!(parsed.timebase(), expected.timebase, "timebase is expected");
        assert_eq!(parsed.ntsc(), case.ntsc, "ntsc is expected");
    }

    #[rstest]
    #[case(Framerate::new_with_timebase(24, Ntsc::False), "[24]")]
    #[case(
        Framerate::new_with_timebase(24, Ntsc::NonDropFrame),
        "[23.98 NTSC NDF]"
    )]
    #[case(Framerate::new_with_timebase(30, Ntsc::DropFrame), "[29.97 NTSC DF]")]
    fn test_framerate_display(
        #[case] rate: Result<Framerate, FramerateParseError>,
        #[case] display_str: &str,
    ) {
        assert!(rate.is_ok(), "framerate was parsed");
        assert_eq!(format!("{}", rate.unwrap()), display_str)
    }
}
