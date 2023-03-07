#[cfg(test)]
mod test {
    use crate::{Framerate, Ntsc, Timecode};
    use num::Rational64;
    use rstest::{fixture, rstest};
    use serde::de::{Error, Visitor};
    use serde::{Deserialize, Deserializer};
    use serde_json;
    use std::fmt::{self, Debug, Formatter};
    use std::fs;
    use std::str::FromStr;

    /// The path-on-disk to the sequence data we are going to run tests against. This
    /// data represents parsed and combined information from a real-world Premiere Pro
    /// sequence output in matchin EDL and FCP7XML cutlists.
    ///
    /// For more information on how this data was produced, see:
    /// https://github.com/opencinemac/test-timelines
    const PATH_TO_SEQUENCE_JSON: &str =
        "zdevelop/tests/test-timelines/PPRO/Many Basic Edits/Many Basic Edits.json";

    /// Since we can't implement deserializers on Foreign types, we need to make a new
    /// type we can implement the deserializer on.
    #[derive(Debug, Clone, Copy)]
    struct MyRat(Rational64);

    impl<'de> Deserialize<'de> for MyRat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let rational_str = deserializer.deserialize_string(RatVisitor {})?;
            let rat = match Rational64::from_str(&rational_str) {
                Ok(parsed) => parsed,
                Err(err) => {
                    return Err(D::Error::custom(format!("error parsing rational: {}", err)))
                }
            };
            Ok(Self(rat))
        }
    }

    /// Used to deserialize strings directly to Rational64 values.
    struct RatVisitor {}

    impl<'de> Visitor<'de> for RatVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            write!(formatter, "a rational string in format 'x/y'")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.to_string())
        }
    }

    /// Type we can use to deserialize an f64 from a json string value.
    #[derive(Debug, Clone, Copy)]
    struct MyDecimal(f64);

    impl<'de> Deserialize<'de> for MyDecimal {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let float = deserializer.deserialize_string(DecimalStringVisitor {})?;
            Ok(Self(float))
        }
    }

    /// Used to deserialize decimal strings into f64 values.
    struct DecimalStringVisitor {}

    impl<'de> Visitor<'de> for DecimalStringVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            write!(formatter, "a decimal string in format 'x.x'")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let float = match v.parse::<f64>() {
                Ok(parsed) => parsed,
                Err(err) => return Err(E::custom(format!("error parsing float: {}", err))),
            };

            Ok(float)
        }
    }

    /// Data about a real-world timecode observed in an NLE (non-linear editor like Premiere Pro)
    /// sequence.
    ///
    /// Fields which are not currently in use have been commented out.
    #[derive(Deserialize)]
    struct TimecodeInfo {
        timebase: i32,
        ntsc: bool,
        drop_frame: bool,
        frame_rate_frac: MyRat,
        timecode: String,
        frame: i64,
        // frame_xml_raw: i64,
        seconds_rational: MyRat,
        // seconds_decimal: MyDecimal,
        ppro_ticks: i64,
        // ppro_ticks_xml_raw: i64,
        feet_and_frames: String,
        runtime: String,
    }

    /// Data about a real-world event observed in an NLE sequence.
    ///
    /// Fields which are not currently in use have been commented out.
    #[derive(Deserialize)]
    struct Event {
        duration_frames: i64,
        source_in: TimecodeInfo,
        source_out: TimecodeInfo,
        record_in: TimecodeInfo,
        record_out: TimecodeInfo,
    }

    /// Data about a real-world sequence from an NLE.
    #[derive(Deserialize)]
    struct Sequence {
        start_time: TimecodeInfo,
        total_duration_frames: i64,
        events: Vec<Event>,
    }

    #[fixture]
    fn test_sequence() -> Sequence {
        let contents = fs::read_to_string(PATH_TO_SEQUENCE_JSON).unwrap();
        serde_json::from_str(&contents).unwrap()
    }

    /// Tests that we can parse the timecode and frame counts found in real-world EDL
    /// and FCP7XML cutlists. By testing on a large set of real-world timecode values,
    /// we can be much more confident in the correctness of our logic.
    #[rstest]
    fn test_event_parsing(test_sequence: Sequence) {
        // First let's check some values we we know, to do some light validation that
        // the data was parsed correctly.
        assert_eq!(
            19560, test_sequence.total_duration_frames,
            "test sequence duration expected"
        );
        assert_eq!(
            Rational64::new(24000, 1001),
            test_sequence.start_time.frame_rate_frac.0
        );
        assert_eq!(215, test_sequence.events.len(), "event count");

        // Test the parsing of the sequence start time.
        check_timecode_info(&test_sequence.start_time, -1, "start time");

        // Iterate over the events and test the four timecode values associated with
        // each one.
        let mut event_num = 0;
        for event in test_sequence.events.iter() {
            event_num += 1;
            check_timecode_info(&event.record_in, event_num, "record in");
            check_timecode_info(&event.record_out, event_num, "record out");
            check_timecode_info(&event.source_in, event_num, "source in");
            check_timecode_info(&event.source_out, event_num, "source out");
        }
    }

    /// checks that we can parse a timecode value from many of it's different
    /// representations.
    fn check_timecode_info(info: &TimecodeInfo, event_num: i32, tc_val: &str) {
        let ntsc = if !info.ntsc {
            Ntsc::None
        } else if info.drop_frame {
            Ntsc::DropFrame
        } else {
            Ntsc::NonDropFrame
        };

        let rate = Framerate::with_timebase(info.timebase, ntsc).unwrap();
        let tc = Timecode::with_frames(&info.timecode, rate).unwrap();
        check_parsed_timecode(tc, info, event_num, tc_val, &info.timecode, "timecode");

        let tc = Timecode::with_frames(&info.frame, rate).unwrap();
        check_parsed_timecode(tc, info, event_num, tc_val, &info.frame, "frame numer");

        let tc = Timecode::with_seconds(&info.seconds_rational.0, rate).unwrap();
        check_parsed_timecode(
            tc,
            info,
            event_num,
            tc_val,
            &info.seconds_rational.0,
            "seconds",
        );

        let tc = Timecode::with_seconds(&info.runtime, rate).unwrap();
        check_parsed_timecode(tc, info, event_num, tc_val, &info.runtime, "runtime");

        let tc = Timecode::with_premiere_ticks(&info.ppro_ticks, rate).unwrap();
        check_parsed_timecode(tc, info, event_num, tc_val, &info.ppro_ticks, "ppro ticks");
    }

    /// check a parsed timecode value against it's expected representations.
    fn check_parsed_timecode<'a, T>(
        tc: Timecode,
        info: &TimecodeInfo,
        event_num: i32,
        tc_field: &str,
        source: &'a T,
        source_type: &str,
    ) where
        T: 'a + Debug,
        &'a T: Debug,
    {
        assert_eq!(
            info.timecode,
            tc.timecode(),
            "event {} {} tc string from {:?} ({})",
            event_num,
            tc_field,
            source,
            source_type,
        );
        assert_eq!(
            info.frame,
            tc.frames(),
            "event {} {} frame number from {:?} ({})",
            event_num,
            tc_field,
            source,
            source_type,
        );
        assert_eq!(
            info.seconds_rational.0,
            tc.seconds(),
            "event {} {} seconds rational from {:?} ({})",
            event_num,
            tc_field,
            source,
            source_type,
        );
        assert_eq!(
            info.feet_and_frames,
            tc.feet_and_frames_35mm_4p(),
            "event {} {} feet and frames from {:?} ({})",
            event_num,
            tc_field,
            source,
            source_type,
        );
        assert_eq!(
            info.runtime,
            tc.runtime(9),
            "event {} {} feet and frames from {:?} ({})",
            event_num,
            tc_field,
            source,
            source_type,
        );
    }

    /// Tests that we can keep accurate running totals of a sequences events.
    #[rstest]
    fn test_sequence_totals(test_sequence: Sequence) {
        // We're goingg to keep a running total of the expected out point here.
        let mut current_out = parse_timecode(&test_sequence.start_time);

        // We're going to store the total frames from adding up all our events here.
        let mut current_total = Timecode::with_frames(0, current_out.rate()).unwrap();

        for event in test_sequence.events.iter() {
            let rec_in = parse_timecode(&event.record_in);
            let rec_out = parse_timecode(&event.record_out);
            let src_in = parse_timecode(&event.source_in);
            let src_out = parse_timecode(&event.source_out);

            let rec_duration = rec_out - rec_in;
            let src_duration: Timecode = src_out - src_in;

            assert_eq!(event.duration_frames, rec_duration.frames(), "rec duration");
            assert_eq!(event.duration_frames, src_duration.frames(), "src duration");

            current_out += src_duration;

            assert_eq!(
                event.record_out.timecode,
                current_out.timecode(),
                "rec out total expected"
            );

            current_total += rec_duration;
        }

        assert_eq!(
            test_sequence.total_duration_frames,
            current_total.frames(),
            "total frames"
        )
    }

    fn parse_timecode(info: &TimecodeInfo) -> Timecode {
        let ntsc = if !info.ntsc {
            Ntsc::None
        } else if info.drop_frame {
            Ntsc::DropFrame
        } else {
            Ntsc::NonDropFrame
        };

        let rate = Framerate::with_timebase(info.timebase, ntsc).unwrap();
        Timecode::with_frames(&info.timecode, rate).unwrap()
    }
}
