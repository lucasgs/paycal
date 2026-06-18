use crate::{parse_args, CliAction, Data};

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

#[test]
fn parse_help_flag() {
    let action = parse_args(["--help".to_string()]).unwrap();
    assert!(matches!(action, CliAction::Help));
}

#[test]
fn parse_rejects_missing_args() {
    let err = parse_args(Vec::<String>::new()).unwrap_err();
    assert_eq!(err, "expected exactly 2 arguments: <rate> <hours_per_day>");
}

#[test]
fn parse_rejects_negative_rate() {
    let err = parse_args(["-1".to_string(), "8".to_string()]).unwrap_err();
    assert_eq!(err, "rate must be non-negative");
}

#[test]
fn parse_rejects_invalid_rate() {
    let err = parse_args(["abc".to_string(), "8".to_string()]).unwrap_err();
    assert_eq!(err, "rate must be a valid number");
}

#[test]
fn parse_rejects_invalid_hours() {
    let err = parse_args(["20".to_string(), "abc".to_string()]).unwrap_err();
    assert_eq!(err, "hours_per_day must be a whole number between 0 and 24");
}

#[test]
fn parse_rejects_unrealistic_hours() {
    let err = parse_args(["20".to_string(), "25".to_string()]).unwrap_err();
    assert_eq!(err, "hours_per_day must be between 0 and 24");
}

#[test]
fn parse_accepts_valid_args() {
    let action = parse_args(["20".to_string(), "8".to_string()]).unwrap();

    match action {
        CliAction::Calculate(data) => {
            assert_eq!(data.rate, 20.0);
            assert_eq!(data.hours_per_day, 8);
        }
        CliAction::Help => panic!("expected calculate action"),
    }
}
