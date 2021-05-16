#[cfg(test)]
mod test {
    use crate::{rates, Framerate, Timecode, TimecodeParseError};
    use rstest::rstest;
    use std::fmt::{Debug, Display};
    use std::ops::{Div, Mul, Rem};

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
            tc1: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            eq: true,
            lt: false,
        }
    )]
    // case 2
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::with_frames("00:59:59:24", rates::F24).unwrap(),
            eq: true,
            lt: false,
        }
    )]
    // case 3
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::with_frames("02:00:00:00", rates::F24).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // case 4
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::with_frames("01:00:00:01", rates::F24).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // case 5
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            tc2: Timecode::with_frames("00:59:59:23", rates::F24).unwrap(),
            eq: false,
            lt: false,
        }
    )]
    // 23.98 ---------
    // case 6
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
            eq: true,
            lt: false,
        }
    )]
    // case 7
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::with_frames("01:00:00:01", rates::F23_98).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // case 8
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("00:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::with_frames("02:00:00:01", rates::F23_98).unwrap(),
            eq: false,
            lt: true,
        }
    )]
    // Mixed fps ------
    // case 9
    #[case(
        ComparisonCase{
            tc1: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
            tc2: Timecode::with_frames("01:00:00:01", rates::F24).unwrap(),
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
                Timecode::with_frames("00:01:00:00", rates::F23_98).unwrap(),
                Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
                Timecode::with_frames("00:00:10:00", rates::F23_98).unwrap(),
            ],
            tcs_out: vec![
                Timecode::with_frames("00:00:10:00", rates::F23_98).unwrap(),
                Timecode::with_frames("00:01:00:00", rates::F23_98).unwrap(),
                Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
            ],
        }
    )]
    fn test_sort_timecodes(#[case] mut case: SortCase) {
        case.tcs_in.sort();

        assert_eq!(case.tcs_out, case.tcs_in, "timecodes sorted correctly.")
    }

    struct ArithmeticCase {
        tc1: String,
        tc2: String,
        expected: String,
    }

    #[rstest]
    #[case(ArithmeticCase{
        tc1: "01:00:00:00".to_string(),
        tc2: "01:00:00:00".to_string(),
        expected: "02:00:00:00".to_string(),
    })]
    #[case(ArithmeticCase{
        tc1: "01:00:00:00".to_string(),
        tc2: "00:00:00:01".to_string(),
        expected: "01:00:00:01".to_string(),
    })]
    #[case(ArithmeticCase{
        tc1: "01:00:00:00".to_string(),
        tc2: "-00:30:00:00".to_string(),
        expected: "00:30:00:00".to_string(),
    })]
    fn test_add(#[case] case: ArithmeticCase) -> Result<(), TimecodeParseError> {
        let tc1 = Timecode::with_frames(case.tc1, rates::F24)?;
        let tc2 = Timecode::with_frames(case.tc2, rates::F24)?;
        let expected = Timecode::with_frames(case.expected, rates::F24)?;

        assert_eq!(tc1 + tc2, expected, "{} + {} == {}", tc1, tc2, expected);

        Ok(())
    }

    #[rstest]
    #[case(ArithmeticCase{
        tc1: "01:00:00:00".to_string(),
        tc2: "01:00:00:00".to_string(),
        expected: "00:00:00:00".to_string(),
    })]
    #[case(ArithmeticCase{
        tc1: "01:00:00:00".to_string(),
        tc2: "00:00:00:01".to_string(),
        expected: "00:59:59:23".to_string(),
    })]
    #[case(ArithmeticCase{
        tc1: "01:00:00:00".to_string(),
        tc2: "-00:30:00:00".to_string(),
        expected: "01:30:00:00".to_string(),
    })]
    fn test_subtract(#[case] case: ArithmeticCase) -> Result<(), TimecodeParseError> {
        let tc1 = Timecode::with_frames(case.tc1, rates::F24)?;
        let tc2 = Timecode::with_frames(case.tc2, rates::F24)?;
        let expected = Timecode::with_frames(case.expected, rates::F24)?;

        assert_eq!(tc1 - tc2, expected, "{} - {} == {}", tc1, tc2, expected);

        Ok(())
    }

    struct MultiplyCase<T>
    where
        Timecode: Mul<T>,
    {
        tc: Timecode,
        multiplier: T,
        expected: <Timecode as Mul<T>>::Output,
    }

    /// tests multiplying timecode
    #[rstest]
    // case 1
    #[case(MultiplyCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24,).unwrap(),
        multiplier: 2,
        expected: Timecode::with_frames("02:00:00:00", rates::F24,).unwrap(),
    })]
    // case 2
    #[case(MultiplyCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24,).unwrap(),
        multiplier: 2.0,
        expected: Timecode::with_frames("02:00:00:00", rates::F24,).unwrap(),
    })]
    // case 3
    #[case(MultiplyCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24,).unwrap(),
        multiplier: 1.5,
        expected: Timecode::with_frames("01:30:00:00", rates::F24,).unwrap(),
    })]
    // case 4
    #[case(MultiplyCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24,).unwrap(),
        multiplier: 0.5,
        expected: Timecode::with_frames("00:30:00:00", rates::F24,).unwrap(),
    })]
    // case 5
    #[case(MultiplyCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24,).unwrap(),
        multiplier: 0.0,
        expected: Timecode::with_frames("00:00:00:00", rates::F24,).unwrap(),
    })]
    // case 6
    #[case(MultiplyCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24,).unwrap(),
        multiplier: 0,
        expected: Timecode::with_frames("00:00:00:00", rates::F24,).unwrap(),
    })]
    // case 7
    #[case(MultiplyCase{
        tc: Timecode::with_frames("00:00:00:00", rates::F24,).unwrap(),
        multiplier: 10,
        expected: Timecode::with_frames("00:00:00:00", rates::F24,).unwrap(),
    })]
    // case 8
    #[case(MultiplyCase{
        tc: Timecode::with_frames("00:00:00:00", rates::F24,).unwrap(),
        multiplier: 10.0,
        expected: Timecode::with_frames("00:00:00:00", rates::F24,).unwrap(),
    })]
    fn test_multiply<T>(#[case] case: MultiplyCase<T>)
    where
        // T must be a timecode multiplier
        Timecode: Mul<T>,
        // T must also implement multiplying by timecode.
        T: Mul<Timecode> + Copy + Debug + Display,
        // The output of multiplying a timecode by T must be comparable to itself.
        <Timecode as Mul<T>>::Output: PartialEq<<Timecode as Mul<T>>::Output>
            // It must also be comparable to the output of multiplying itself by timecode.
            + PartialEq<<T as Mul<Timecode>>::Output>
            + Debug
            + Display
            + Copy,
        // The output of multiplying T by our timecode must implement Debug.
        <T as Mul<Timecode>>::Output: Debug,
    {
        let result = case.tc * case.multiplier;
        assert_eq!(case.expected, result, " {} x {}", case.tc, case.multiplier);

        let result = case.multiplier * case.tc;
        assert_eq!(
            case.expected, result,
            "{} x {} (flipped)",
            case.multiplier, case.tc
        );
    }

    struct DivRemCase<T>
    where
        Timecode: Div<T>,
        Timecode: Rem<T>,
    {
        tc: Timecode,
        divisor: T,
        expected_div: <Timecode as Div<T>>::Output,
        expected_rem: <Timecode as Rem<T>>::Output,
    }

    /// tests that our divide and modulo operators yield results that would be expected together.
    #[rstest]
    // case 1
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
        divisor: 2,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
        divisor: 2.0,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
        divisor: 2,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F23_98).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
        divisor: 2.0,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F23_98).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:01", rates::F24).unwrap(),
        divisor: 2,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:01", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:01", rates::F24).unwrap(),
        divisor: 2.0,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:01", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:01", rates::F23_98).unwrap(),
        divisor: 2,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:01", rates::F23_98).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:01", rates::F23_98).unwrap(),
        divisor: 2.0,
        expected_div: Timecode::with_frames("00:30:00:00", rates::F23_98).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:01", rates::F23_98).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
        divisor: 4,
        expected_div: Timecode::with_frames("00:15:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
        divisor: 4.0,
        expected_div: Timecode::with_frames("00:15:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
        divisor: 4,
        expected_div: Timecode::with_frames("00:15:00:00", rates::F23_98).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F23_98).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:00", rates::F23_98).unwrap(),
        divisor: 4.0,
        expected_div: Timecode::with_frames("00:15:00:00", rates::F23_98).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:00", rates::F23_98).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:03", rates::F24).unwrap(),
        divisor: 4,
        expected_div: Timecode::with_frames("00:15:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:03", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:03", rates::F24).unwrap(),
        divisor: 4.0,
        expected_div: Timecode::with_frames("00:15:00:00", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:03", rates::F24).unwrap(),
    })]
    #[case(DivRemCase{
        tc: Timecode::with_frames("01:00:00:4", rates::F24).unwrap(),
        divisor: 1.5,
        expected_div: Timecode::with_frames("00:40:00:02", rates::F24).unwrap(),
        expected_rem: Timecode::with_frames("00:00:00:01", rates::F24).unwrap(),
    })]
    fn test_divrem<T>(#[case] case: DivRemCase<T>)
    where
        T: Display + Debug + Copy,
        // T must be a timecode divisor.
        Timecode: Div<T>,
        // T must be a timecode modulo.
        Timecode: Rem<T>,
        // The output of dividing a timecode by T must be comparable to itself.
        <Timecode as Div<T>>::Output:
            PartialEq<<Timecode as Div<T>>::Output> + Debug + Display + Copy,
        // The output of moduloing a timecode by t must be comparable to itself.
        <Timecode as Rem<T>>::Output:
            PartialEq<<Timecode as Rem<T>>::Output> + Debug + Display + Copy,
    {
        let result_div = case.tc / case.divisor;
        assert_eq!(
            case.expected_div, result_div,
            " {} / {:?} = {}",
            case.tc, case.divisor, case.expected_div
        );

        let result_rem = case.tc % case.divisor;
        assert_eq!(
            case.expected_rem, result_rem,
            " {} % {:?} = {}",
            case.tc, case.divisor, case.expected_rem
        );
    }

    /// test_negative tests the Neg operator (-value).
    #[test]
    fn test_negative() {
        assert_eq!(
            -Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            Timecode::with_frames("-01:00:00:00", rates::F24).unwrap(),
            "neg positive",
        );

        assert_eq!(
            -Timecode::with_frames("-01:00:00:00", rates::F24).unwrap(),
            Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            "neg negative",
        )
    }

    /// test_abs tests our absolute value method.
    #[test]
    fn test_abs() {
        assert_eq!(
            Timecode::with_frames("01:00:00:00", rates::F24)
                .unwrap()
                .abs(),
            Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            "abs positive",
        );

        assert_eq!(
            Timecode::with_frames("-01:00:00:00", rates::F24)
                .unwrap()
                .abs(),
            Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
            "abs negative",
        )
    }

    struct RebaseCase {
        tc_in: Timecode,
        new_rate: Framerate,
        expected: Timecode,
    }

    #[rstest]
    #[case(RebaseCase{
        tc_in: Timecode::with_frames("01:00:00:00", rates::F24).unwrap(),
        new_rate: rates::F48,
        expected: Timecode::with_frames("00:30:00:00", rates::F48).unwrap(),
    })]
    fn test_rebase(#[case] case: RebaseCase) {
        let rebased = case.tc_in.rebase(case.new_rate);
        assert_eq!(case.expected, rebased, "rebased value")
    }
}
