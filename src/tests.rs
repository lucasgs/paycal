#[cfg(test)]
pub mod tests {
    use crate::Data;

    #[test]
    fn dummy_test() {
        assert_eq!(4, 2 + 2)
    }

    #[test]
    fn calculate_round_case() {
        let data = Data {
            rate: 10.0,
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 10.0);
        assert_eq!(res.weekly, 400.0);
        assert_eq!(res.monthly, 1733.3333333333333);
        assert_eq!(res.yearly, 20800.0);
    }

    #[test]
    fn calculate_complex_case() {
        let data = Data {
            rate: 39.5,
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 39.5);
        assert_eq!(res.weekly, 1580.0);
        assert_eq!(res.monthly, 6846.666666666666);
        assert_eq!(res.yearly, 82160.0);
    }

    #[test]
    fn calculate_zero_rate_case() {
        let data = Data {
            rate: 0.0,
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 0.0);
        assert_eq!(res.weekly, 0.0);
        assert_eq!(res.monthly, 0.0);
        assert_eq!(res.yearly, 0.0);
    }

    #[test]
    fn calculate_zero_hours_case() {
        let data = Data {
            rate: 10.0,
            hours_per_day: 0,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 10.0);
        assert_eq!(res.weekly, 0.0);
        assert_eq!(res.monthly, 0.0);
        assert_eq!(res.yearly, 0.0);
    }
}
