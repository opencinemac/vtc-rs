#[cfg(test)]
mod test {
    use crate::{rates, Timecode};
    use rstest::rstest;
    use std::fmt::Debug;
    use std::ops::Mul;

    struct ComparisonCase {
        tc1: Timecode,
        tc2: Timecode,
        eq: bool,
        lt: bool,
    }

    /// tests comparisons
    #[rstest]
    // 24 FPS ---------
    // case 1
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::new_with_frames("01:00:00:00", rates::F24).unwrap(),
            eq: true,
            lt: false,
        }
    )]
    // case 2
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::new_with_frames("00:59:59:24", rates::F24).unwrap(),
            eq: true,
            lt: false,
        }
    )]
    // case 3
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::new_with_frames("02:00:00:00", rates::F24).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // case 4
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::new_with_frames("01:00:00:01", rates::F24).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // case 5
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::new_with_frames("00:59:59:23", rates::F24).unwrap(),
            eq: false,
            lt: false,
        }
    )]
    // 23.98 ---------
    // case 6
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::new_with_frames("01:00:00:00", rates::F23_98).unwrap(),
            eq: true,
            lt: false,
        }
    )]
    // case 7
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::new_with_frames("01:00:00:01", rates::F23_98).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // case 8
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("00:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::new_with_frames("02:00:00:01", rates::F23_98).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // Mixed fps ------
    // case 9
    #[case(
        ComparisonCase{
            tc1: Timecode::new_with_frames("01:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::new_with_frames("01:00:00:01", rates::F24).unwrap(),
            eq: false,
            lt: false,
        }
    )]
    fn test_comparison(#[case] case: ComparisonCase) {
        // eq
        assert_eq!(
            case.eq,
            case.tc1 == case.tc2,
            "{} == {}",
            case.tc1,
            case.tc2
        );

        // eq flipped
        assert_eq!(
            case.eq,
            case.tc2 == case.tc1,
            "{} == {} (flipped)",
            case.tc2,
            case.tc1
        );

        // lt
        assert_eq!(case.lt, case.tc1 < case.tc2, "{} < {}", case.tc1, case.tc2);

        // we can use the expected eq and lt values to derive our expected values for the rest
        // of the comparisons.

        // lt flipped
        let mut expected = !case.lt && !case.eq;
        assert_eq!(
            expected,
            case.tc2 < case.tc1,
            "{} < {} (flipped)",
            case.tc2,
            case.tc1
        );

        // lte
        expected = case.lt || case.eq;
        assert_eq!(
            expected,
            case.tc1 <= case.tc2,
            "{} <= {}",
            case.tc1,
            case.tc2
        );

        // lte flipped
        expected = !case.lt || case.eq;
        assert_eq!(
            expected,
            case.tc2 <= case.tc1,
            "{} <= {} (flipped)",
            case.tc2,
            case.tc1
        );

        // gt
        expected = !case.eq && !case.lt;
        assert_eq!(expected, case.tc1 > case.tc2, "{} > {}", case.tc1, case.tc2);

        // gt flipped
        expected = case.lt;
        assert_eq!(
            expected,
            case.tc2 > case.tc1,
            "{} > {} (flipped)",
            case.tc2,
            case.tc1
        );

        // gte
        expected = case.eq || !case.lt;
        assert_eq!(
            expected,
            case.tc1 >= case.tc2,
            "{} >= {}",
            case.tc1,
            case.tc2
        );

        // gte flipped
        expected = case.eq || case.lt;
        assert_eq!(
            expected,
            case.tc2 >= case.tc1,
            "{} >= {} (flipped)",
            case.tc2,
            case.tc1
        );
    }

    struct SortCase {
        tcs_in: Vec<Timecode>,
        tcs_out: Vec<Timecode>,
    }

    /// tests that timecode comparisons lead to expected sorting behavior.
    #[rstest]
    #[case(
        SortCase {
            tcs_in: vec![
                Timecode::new_with_frames("00:01:00:00", rates::F23_98).unwrap(),
                Timecode::new_with_frames("01:00:00:00", rates::F23_98).unwrap(),
                Timecode::new_with_frames("00:00:10:00", rates::F23_98).unwrap(),
            ],
            tcs_out: vec![
                Timecode::new_with_frames("00:00:10:00", rates::F23_98).unwrap(),
                Timecode::new_with_frames("00:01:00:00", rates::F23_98).unwrap(),
                Timecode::new_with_frames("01:00:00:00", rates::F23_98).unwrap(),
            ],
        }
    )]
    fn test_sort_timecodes(#[case] mut case: SortCase) {
        case.tcs_in.sort();

        assert_eq!(case.tcs_out, case.tcs_in, "timecodes sorted correctly.")
    }

    struct MultiplyCase<T>
    where
        Timecode: Mul<T>,
    {
        tc: Timecode,
        multiplier: T,
        expected: <Timecode as Mul<T>>::Output,
    }

    fn test_multiply<T>(case: MultiplyCase<T>)
    where
        Timecode: Mul<T>,
        <Timecode as Mul<T>>::Output: PartialEq<<Timecode as Mul<T>>::Output>
            + Debug
            + Copy
            + PartialEq<<T as Mul<Timecode>>::Output>,
        T: Copy + Mul<Timecode>,
        <T as Mul<Timecode>>::Output: Debug,
    {
        let result = case.tc * case.multiplier;
        assert_eq!(case.expected, result, "multiplication result");

        let result = case.multiplier * case.tc;
        let b = case.expected == result;
        assert_eq!(case.expected, result, "multiplication result");
    }
}
