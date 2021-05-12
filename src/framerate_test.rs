#[cfg(test)]
mod test {
    use crate::{Framerate, Ntsc};
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
    struct ParseCase {
        /// source is the source framerate value to parse frome.
        source: ParseSource,
        /// source_type is whether the source if a framerate or timebase.
        source_type: SourceType,
        /// ntsc is whether the source should be parsed as an NTSC framerate.
        ntsc: Ntsc,
        /// expected is the expected result. Use Err([message])
        expected: Result<Success, String>,
    }

    /// ParseSource is the source type we are going to test parsing
    enum ParseSource {
        Rational(num::Rational64),
        Str(String),
        Int(i64),
        Float(f64),
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
        source: ParseSource::Float(23.98),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From whole float rounded
    #[case::float_2398_ntsc_timebase(ParseCase{
        source: ParseSource::Float(24.0),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From non-whole float string
    #[case::strfloat_2398_ntsc_plaback(ParseCase{
        source: ParseSource::Str("23.98".to_string()),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From float string
    #[case::strfloat_2398_ntsc_base(ParseCase{
        source: ParseSource::Str("24.0".to_string()),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From int.
    #[case::int_2398_ntsc_timebase(ParseCase{
        source: ParseSource::Int(24),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From rational non-whole.
    #[case::rational_2398_ntsc_playback(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24000, 1001)),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // NTSC From rational whole number.
    #[case::rational_2398_ntsc_timebase(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24, 1)),
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
        source: ParseSource::Int(24),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    #[case::int_24_playback(ParseCase{
        source: ParseSource::Int(24),
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })]
    // From rational non-whole.
    #[case::rational_2398_playback(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24000, 1001)),
        source_type: SourceType::Playback,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24000, 1001),
        }),
    })]
    // From rational non-whole.
    #[case::rational_2398_timebase(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24000, 1001)),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24000, 1001),
            timebase: num::Rational64::new(24000, 1001),
        }),
    })]
    // From rational whole number.
    #[case::rational_2398_timebase(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24, 1)),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::False,
        expected: Ok(Success{
            playback: num::Rational64::new(24, 1),
            timebase: num::Rational64::new(24, 1),
        }),
    })] // From rational whole number.
    #[case::rational_2398_playback(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24, 1)),
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
        source: ParseSource::Float(29.97),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 float - whole.
    #[case::float_2997_drop_timebase(ParseCase{
        source: ParseSource::Float(30.0),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 int.
    #[case::int_2997_drop_timebase(ParseCase{
        source: ParseSource::Int(30),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 rational - non-whole.
    #[case::rational_2997_drop_playback(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(30000, 1001)),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 rational - whole.
    #[case::rational_2997_drop_timebase(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(30, 1)),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string float non-whole.
    #[case::strfloat_2997_drop_playback(ParseCase{
        source: ParseSource::Str("29.97".to_string()),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string float whole.
    #[case::strfloat_2997_drop_timebase(ParseCase{
        source: ParseSource::Str("30.0".to_string()),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string int.
    #[case::strint_2997_drop_timebase(ParseCase{
        source: ParseSource::Str("30".to_string()),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string rational non-whole.
    #[case::strrational_2997_drop_playback(ParseCase{
        source: ParseSource::Str("30000/1001".to_string()),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 29.97 string rational whole.
    #[case::strrational_2997_drop_timebase(ParseCase{
        source: ParseSource::Str("30/1".to_string()),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(30000, 1001),
            timebase: num::Rational64::new(30, 1),
        }),
    })]
    // Drop-frame 59.94 float non-whole.
    #[case::float_5994_drop_playback(ParseCase{
        source: ParseSource::Float(59.94),
        source_type: SourceType::Playback,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(60000, 1001),
            timebase: num::Rational64::new(60, 1),
        }),
    })]
    // Drop-frame 59.94 int
    #[case::float_5994_drop_timebase(ParseCase{
        source: ParseSource::Int(60),
        source_type: SourceType::Timebase,
        ntsc: Ntsc::DropFrame,
        expected: Ok(Success{
            playback: num::Rational64::new(60000, 1001),
            timebase: num::Rational64::new(60, 1),
        }),
    })]
    // ERROR CASES ---------
    // ---------------------
    #[case::error_ntsc_playback_bad_denom(ParseCase{
        source: ParseSource::Rational(num::Rational64::new(24, 1)),
        source_type: SourceType::Playback,
        ntsc: Ntsc::NonDropFrame,
        expected: Err("ntsc framerates must be n/1001".to_string()),
    })]
    fn test_parse_framerate(#[case] case: ParseCase) {
        let result = match case.source_type {
            SourceType::Playback => match case.source {
                ParseSource::Rational(source) => {
                    Framerate::from_rational_playback(source, case.ntsc)
                }
                ParseSource::Str(source) => Framerate::from_str_playback(&source, case.ntsc),
                ParseSource::Int(source) => Framerate::from_i64_playback(source),
                ParseSource::Float(source) => Framerate::from_f64_playback(source, case.ntsc),
            },
            SourceType::Timebase => match case.source {
                ParseSource::Rational(source) => {
                    Framerate::from_rational_timebase(source, case.ntsc)
                }
                ParseSource::Str(source) => Framerate::from_str_timebase(&source, case.ntsc),
                ParseSource::Int(source) => Framerate::from_i64_timebase(source, case.ntsc),
                ParseSource::Float(source) => Framerate::from_f64_timebase(source, case.ntsc),
            },
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
    #[case(Framerate::from_i64_timebase(24, Ntsc::False), "[24]")]
    #[case(Framerate::from_i64_timebase(24, Ntsc::NonDropFrame), "[23.98 NTSC]")]
    #[case(Framerate::from_i64_timebase(30, Ntsc::DropFrame), "[29.97 NTSC DF]")]
    fn test_framerate_display(#[case] rate: Result<Framerate, String>, #[case] display_str: &str) {
        assert!(rate.is_ok(), "framerate was parsed");
        assert_eq!(format!("{}", rate.unwrap()), display_str)
    }
}
