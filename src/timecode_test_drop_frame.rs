#[cfg(test)]
mod test {
    use crate::{rates, Framerate, Ntsc, Timecode};
    use rstest::rstest;

    /// There are no constants for 119.88 or 239.76, so build them the way a caller would.
    fn f119_88(ntsc: Ntsc) -> Framerate {
        Framerate::with_playback("119.88", ntsc).unwrap()
    }

    fn f239_76(ntsc: Ntsc) -> Framerate {
        Framerate::with_playback("239.76", ntsc).unwrap()
    }

    struct DropFrameCase {
        drop_rate: Framerate,
        non_drop_rate: Framerate,
        frames: i64,
        drop_timecode: &'static str,
        non_drop_timecode: &'static str,
    }

    /// Tests that frame counts render to the correct drop-frame timecode, that the timecode
    /// parses back to the same frame count, and that the internal representation matches the
    /// equivalent non-drop-frame value.
    #[rstest]
    // 29.97 DF ---------
    // Partway through the first minute. Previously rendered as 00:00:30;04.
    #[case::f29_97_t00_00_30_03(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 903,
        drop_timecode: "00:00:30;03",
        non_drop_timecode: "00:00:30:03",
    })]
    // Previously rendered as 00:00:33;11.
    #[case::f29_97_t00_00_33_10(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 1000,
        drop_timecode: "00:00:33;10",
        non_drop_timecode: "00:00:33:10",
    })]
    // Last frame before the first drop.
    #[case::f29_97_t00_00_59_29(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 1799,
        drop_timecode: "00:00:59;29",
        non_drop_timecode: "00:00:59:29",
    })]
    // First frame after the first drop.
    #[case::f29_97_t00_01_00_02(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 1800,
        drop_timecode: "00:01:00;02",
        non_drop_timecode: "00:01:00:00",
    })]
    // Ten-minute marks do not drop frames.
    #[case::f29_97_t00_10_00_00(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 17982,
        drop_timecode: "00:10:00;00",
        non_drop_timecode: "00:09:59:12",
    })]
    #[case::f29_97_t00_10_00_01(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 17983,
        drop_timecode: "00:10:00;01",
        non_drop_timecode: "00:09:59:13",
    })]
    #[case::f29_97_t00_10_59_29(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 19781,
        drop_timecode: "00:10:59;29",
        non_drop_timecode: "00:10:59:11",
    })]
    #[case::f29_97_t00_11_00_02(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 19782,
        drop_timecode: "00:11:00;02",
        non_drop_timecode: "00:10:59:12",
    })]
    #[case::f29_97_t00_59_59_29(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 107891,
        drop_timecode: "00:59:59;29",
        non_drop_timecode: "00:59:56:11",
    })]
    #[case::f29_97_t01_00_00_00(DropFrameCase{
        drop_rate: rates::F29_97_DF,
        non_drop_rate: rates::F29_97_NDF,
        frames: 107892,
        drop_timecode: "01:00:00;00",
        non_drop_timecode: "00:59:56:12",
    })]
    // 59.94 DF ---------
    // Previously rendered as 00:00:15;04.
    #[case::f59_94_t00_00_15_03(DropFrameCase{
        drop_rate: rates::F59_94_DF,
        non_drop_rate: rates::F59_94_NDF,
        frames: 903,
        drop_timecode: "00:00:15;03",
        non_drop_timecode: "00:00:15:03",
    })]
    #[case::f59_94_t00_00_59_59(DropFrameCase{
        drop_rate: rates::F59_94_DF,
        non_drop_rate: rates::F59_94_NDF,
        frames: 3599,
        drop_timecode: "00:00:59;59",
        non_drop_timecode: "00:00:59:59",
    })]
    #[case::f59_94_t00_01_00_04(DropFrameCase{
        drop_rate: rates::F59_94_DF,
        non_drop_rate: rates::F59_94_NDF,
        frames: 3600,
        drop_timecode: "00:01:00;04",
        non_drop_timecode: "00:01:00:00",
    })]
    #[case::f59_94_t00_10_00_00(DropFrameCase{
        drop_rate: rates::F59_94_DF,
        non_drop_rate: rates::F59_94_NDF,
        frames: 35964,
        drop_timecode: "00:10:00;00",
        non_drop_timecode: "00:09:59:24",
    })]
    #[case::f59_94_t00_10_00_02(DropFrameCase{
        drop_rate: rates::F59_94_DF,
        non_drop_rate: rates::F59_94_NDF,
        frames: 35966,
        drop_timecode: "00:10:00;02",
        non_drop_timecode: "00:09:59:26",
    })]
    #[case::f59_94_t01_00_00_00(DropFrameCase{
        drop_rate: rates::F59_94_DF,
        non_drop_rate: rates::F59_94_NDF,
        frames: 215784,
        drop_timecode: "01:00:00;00",
        non_drop_timecode: "00:59:56:24",
    })]
    // 119.88 DF ---------
    // Previously rendered as 00:00:30;03.
    #[case::f119_88_t00_00_30_00(DropFrameCase{
        drop_rate: f119_88(Ntsc::DropFrame),
        non_drop_rate: f119_88(Ntsc::NonDropFrame),
        frames: 3600,
        drop_timecode: "00:00:30;00",
        non_drop_timecode: "00:00:30:00",
    })]
    // First frame after the first drop. Previously rendered as 00:01:00;07.
    #[case::f119_88_t00_01_00_08(DropFrameCase{
        drop_rate: f119_88(Ntsc::DropFrame),
        non_drop_rate: f119_88(Ntsc::NonDropFrame),
        frames: 7200,
        drop_timecode: "00:01:00;08",
        non_drop_timecode: "00:01:00:00",
    })]
    #[case::f119_88_t01_00_00_00(DropFrameCase{
        drop_rate: f119_88(Ntsc::DropFrame),
        non_drop_rate: f119_88(Ntsc::NonDropFrame),
        frames: 431568,
        drop_timecode: "01:00:00;00",
        non_drop_timecode: "00:59:56:48",
    })]
    // 239.76 DF ---------
    #[case::f239_76_t00_01_00_16(DropFrameCase{
        drop_rate: f239_76(Ntsc::DropFrame),
        non_drop_rate: f239_76(Ntsc::NonDropFrame),
        frames: 14400,
        drop_timecode: "00:01:00;16",
        non_drop_timecode: "00:01:00:00",
    })]
    fn test_drop_frame_values(#[case] case: DropFrameCase) {
        let from_frames = Timecode::with_frames(case.frames, case.drop_rate).unwrap();
        assert_eq!(
            case.drop_timecode,
            from_frames.timecode(),
            "frames {} rendered to the wrong drop-frame timecode",
            case.frames,
        );

        let from_drop_tc = Timecode::with_frames(case.drop_timecode, case.drop_rate).unwrap();
        assert_eq!(
            case.frames,
            from_drop_tc.frames(),
            "{} parsed to the wrong frame count",
            case.drop_timecode,
        );

        let from_non_drop_tc =
            Timecode::with_frames(case.non_drop_timecode, case.non_drop_rate).unwrap();
        assert_eq!(
            from_non_drop_tc.frames(),
            from_drop_tc.frames(),
            "{} and {} should be the same frame",
            case.drop_timecode,
            case.non_drop_timecode,
        );
        assert_eq!(
            from_non_drop_tc.seconds(),
            from_drop_tc.seconds(),
            "{} and {} should be the same instant",
            case.drop_timecode,
            case.non_drop_timecode,
        );
        assert_eq!(case.non_drop_timecode, from_non_drop_tc.timecode());
    }

    /// Tests that drop-frame timecodes render and parse back to the same frame count, and
    /// that consecutive frames only skip the frame numbers drop-frame is allowed to skip.
    ///
    /// The drop pattern repeats every 10 minutes, so every frame is checked through the
    /// first two minutes (a minute that does not drop, then one that does) and around the
    /// 10-minute mark. A prime stride then samples frames across 24 hours, landing at a
    /// different position within each minute.
    #[rstest]
    #[case::f29_97(rates::F29_97_DF)]
    #[case::f59_94(rates::F59_94_DF)]
    #[case::f119_88(f119_88(Ntsc::DropFrame))]
    fn test_drop_frame_round_trip(#[case] rate: Framerate) {
        let timebase = rate.timebase().to_integer();
        let dropped_per_min = rate.drop_frames_per_minute().unwrap();
        let frames_per_full_min = timebase * 60;
        let frames_per_min = frames_per_full_min - dropped_per_min;
        let frames_per_10_min = frames_per_min * 10 + dropped_per_min;
        let frames_per_24_hours = frames_per_10_min * 6 * 24;

        let first_two_minutes = 0..=frames_per_full_min + frames_per_min;
        let around_ten_minutes =
            frames_per_10_min - frames_per_min..=frames_per_10_min + frames_per_full_min;
        let across_24_hours = (0..frames_per_24_hours).step_by(1009);

        let mut previous: Option<(i64, i64, i64)> = None;
        for frame in first_two_minutes
            .chain(around_ten_minutes)
            .chain(across_24_hours)
        {
            let tc = Timecode::with_frames(frame, rate).unwrap();
            let rendered = tc.timecode();
            let parsed = Timecode::with_frames(rendered.as_str(), rate).unwrap();
            assert_eq!(
                frame,
                parsed.frames(),
                "frame {} rendered as {}",
                frame,
                rendered
            );

            let sections = tc.sections();
            let minute = sections.hours * 60 + sections.minutes;
            let label = sections.seconds * timebase + sections.frames;

            if let Some((prev_frame, prev_minute, prev_label)) = previous {
                if prev_frame + 1 == frame && prev_minute == minute {
                    assert_eq!(
                        prev_label + 1,
                        label,
                        "frame {} ({}) skipped a label",
                        frame,
                        rendered
                    );
                } else if prev_frame + 1 == frame {
                    let expected = if minute % 10 == 0 { 0 } else { dropped_per_min };
                    assert_eq!(
                        expected, label,
                        "frame {} ({}) started the minute on the wrong label",
                        frame, rendered,
                    );
                }
            }
            previous = Some((frame, minute, label));
        }
    }
}
