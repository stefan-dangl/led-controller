pub fn number_of_active_wifi_bars(signal_strength: i8, number_of_bars: i8) -> i8 {
    let inactive_bars = signal_strength / (i8::MIN / number_of_bars);
    number_of_bars - inactive_bars
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_case::test_case(0, 4, 4)]
    #[test_case::test_case(-30, 4, 4)]
    #[test_case::test_case(-60, 4, 3)]
    #[test_case::test_case(-90, 4, 2)]
    #[test_case::test_case(-100, 4, 1)]
    #[test_case::test_case(-128, 4, 0)]
    fn test_number_of_active_wifi_bars(signal_strength: i8, number_of_bars: i8, expected: i8) {
        let result = number_of_active_wifi_bars(signal_strength, number_of_bars);
        assert_eq!(result, expected);
    }
}
