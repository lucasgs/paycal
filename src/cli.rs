use std::env;

use crate::{PayInput, WorkSchedule};

pub const USAGE: &str = "paycal - CLI pay calculator\n\nUsage:\n  paycal <rate> <hours_per_day> [days_per_week] [weeks_per_year] [months_per_year]\n  paycal --help\n\nArguments:\n  <rate>             Hourly pay rate (must be non-negative)\n  <hours_per_day>    Hours worked per day (must be between 0 and 24)\n  [days_per_week]    Optional work days per week (must be greater than 0)\n  [weeks_per_year]   Optional work weeks per year (must be greater than 0)\n  [months_per_year]  Optional months per year (must be greater than 0)\n\nExamples:\n  paycal 20 8\n  paycal 20 8 4 48 12\n  cargo run -- 20 8 4 48 12";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CliAction {
    Help,
    Calculate {
        input: PayInput,
        schedule: WorkSchedule,
    },
}

/// Reads command-line arguments from the current process and parses them.
pub fn read_args() -> Result<CliAction, String> {
    parse_args(env::args().skip(1))
}

/// Parses CLI arguments into either a help action or validated pay input.
pub fn parse_args<I>(args: I) -> Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();

    if args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h") {
        return Ok(CliAction::Help);
    }

    if !(2..=5).contains(&args.len()) {
        return Err(
            "expected 2 required arguments and up to 3 optional ones: <rate> <hours_per_day> [days_per_week] [weeks_per_year] [months_per_year]".to_string(),
        );
    }

    let rate: f64 = args[0]
        .parse()
        .map_err(|_| "rate must be a valid number".to_string())?;
    if !rate.is_finite() {
        return Err("rate must be a finite number".to_string());
    }
    if rate < 0.0 {
        return Err("rate must be non-negative".to_string());
    }

    let hours_per_day: u8 = args[1]
        .parse()
        .map_err(|_| "hours_per_day must be a whole number between 0 and 24".to_string())?;
    if hours_per_day > 24 {
        return Err("hours_per_day must be between 0 and 24".to_string());
    }

    let schedule = WorkSchedule {
        days_per_week: parse_positive_f64(args.get(2), "days_per_week")?.unwrap_or(5.0),
        weeks_per_year: parse_positive_f64(args.get(3), "weeks_per_year")?.unwrap_or(52.0),
        months_per_year: parse_positive_f64(args.get(4), "months_per_year")?.unwrap_or(12.0),
    };

    Ok(CliAction::Calculate {
        input: PayInput {
            rate,
            hours_per_day,
        },
        schedule,
    })
}

fn parse_positive_f64(value: Option<&String>, name: &str) -> Result<Option<f64>, String> {
    let Some(value) = value else {
        return Ok(None);
    };

    let parsed: f64 = value
        .parse()
        .map_err(|_| format!("{name} must be a valid number"))?;

    if !parsed.is_finite() {
        return Err(format!("{name} must be a finite number"));
    }
    if parsed <= 0.0 {
        return Err(format!("{name} must be greater than 0"));
    }

    Ok(Some(parsed))
}

#[cfg(test)]
mod tests {
    use super::{parse_args, CliAction};
    use crate::{PayInput, WorkSchedule};

    #[test]
    fn parse_help_flag() {
        let action = parse_args(["--help".to_string()]).unwrap();
        assert!(matches!(action, CliAction::Help));
    }

    #[test]
    fn parse_rejects_missing_args() {
        let err = parse_args(Vec::<String>::new()).unwrap_err();
        assert_eq!(
            err,
            "expected 2 required arguments and up to 3 optional ones: <rate> <hours_per_day> [days_per_week] [weeks_per_year] [months_per_year]"
        );
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
    fn parse_accepts_valid_args_with_defaults() {
        let action = parse_args(["20".to_string(), "8".to_string()]).unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                input: PayInput {
                    rate: 20.0,
                    hours_per_day: 8,
                },
                schedule: WorkSchedule::default(),
            }
        );
    }

    #[test]
    fn parse_accepts_custom_schedule() {
        let action = parse_args([
            "20".to_string(),
            "8".to_string(),
            "4".to_string(),
            "48".to_string(),
            "12".to_string(),
        ])
        .unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                input: PayInput {
                    rate: 20.0,
                    hours_per_day: 8,
                },
                schedule: WorkSchedule {
                    days_per_week: 4.0,
                    weeks_per_year: 48.0,
                    months_per_year: 12.0,
                },
            }
        );
    }

    #[test]
    fn parse_rejects_invalid_days_per_week() {
        let err = parse_args(["20".to_string(), "8".to_string(), "0".to_string()]).unwrap_err();
        assert_eq!(err, "days_per_week must be greater than 0");
    }

    #[test]
    fn parse_rejects_invalid_weeks_per_year() {
        let err = parse_args([
            "20".to_string(),
            "8".to_string(),
            "5".to_string(),
            "0".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "weeks_per_year must be greater than 0");
    }

    #[test]
    fn parse_rejects_invalid_months_per_year() {
        let err = parse_args([
            "20".to_string(),
            "8".to_string(),
            "5".to_string(),
            "52".to_string(),
            "0".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "months_per_year must be greater than 0");
    }
}
