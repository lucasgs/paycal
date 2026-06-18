use std::env;

use clap::{error::ErrorKind, CommandFactory, Parser, ValueEnum};
use rust_decimal::Decimal;

use crate::{PayInput, WorkSchedule};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Table,
    Csv,
    Json,
}

#[derive(Debug, Clone, PartialEq, Parser)]
#[command(
    name = "paycal",
    version,
    about = "CLI pay calculator",
    long_about = None,
    after_help = "Examples:\n  paycal --rate 20 --hours 8\n  paycal --rate 20,25,30 --hours 8\n  paycal --rate 20,25 --hours 8 --days-per-week 4 --weeks-per-year 48 --months-per-year 12\n  paycal --rate 20,25 --hours 8 --format csv\n  paycal --rate 20,25 --hours 8 --format json\n  paycal 20,25 8\n  cargo run -- --rate 20,25,30 --hours 8"
)]
struct CliArgs {
    #[arg(long, value_name = "RATE[,RATE...]", allow_hyphen_values = true)]
    rate: Option<String>,
    #[arg(long = "hours", value_name = "HOURS_PER_DAY")]
    hours_per_day: Option<u8>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,
    #[arg(
        long = "days-per-week",
        value_name = "DAYS",
        allow_hyphen_values = true
    )]
    days_per_week: Option<Decimal>,
    #[arg(
        long = "weeks-per-year",
        value_name = "WEEKS",
        allow_hyphen_values = true
    )]
    weeks_per_year: Option<Decimal>,
    #[arg(
        long = "months-per-year",
        value_name = "MONTHS",
        allow_hyphen_values = true
    )]
    months_per_year: Option<Decimal>,
    #[arg(value_name = "RATE[,RATE...]", hide = true, allow_hyphen_values = true)]
    positional_rate: Option<String>,
    #[arg(value_name = "HOURS_PER_DAY", hide = true)]
    positional_hours_per_day: Option<u8>,
    #[arg(value_name = "DAYS_PER_WEEK", hide = true, allow_hyphen_values = true)]
    positional_days_per_week: Option<Decimal>,
    #[arg(value_name = "WEEKS_PER_YEAR", hide = true, allow_hyphen_values = true)]
    positional_weeks_per_year: Option<Decimal>,
    #[arg(
        value_name = "MONTHS_PER_YEAR",
        hide = true,
        allow_hyphen_values = true
    )]
    positional_months_per_year: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CliAction {
    Help,
    Calculate {
        inputs: Vec<PayInput>,
        schedule: WorkSchedule,
        format: OutputFormat,
    },
}

pub fn usage() -> String {
    let mut command = CliArgs::command();
    command.render_long_help().to_string()
}

pub fn read_args() -> Result<CliAction, String> {
    parse_args(env::args().skip(1))
}

pub fn parse_args<I>(args: I) -> Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let argv: Vec<String> = std::iter::once("paycal".to_string()).chain(args).collect();

    let parsed = match CliArgs::try_parse_from(argv) {
        Ok(parsed) => parsed,
        Err(error) if error.kind() == ErrorKind::DisplayHelp => return Ok(CliAction::Help),
        Err(error) => return Err(error.to_string().trim().to_string()),
    };

    let rate_text = parsed.rate.or(parsed.positional_rate).ok_or_else(|| {
        "either --rate <RATE[,RATE...]> or positional <RATE[,RATE...]> is required".to_string()
    })?;
    let rates = parse_rates(&rate_text)?;

    let hours_per_day = parsed
        .hours_per_day
        .or(parsed.positional_hours_per_day)
        .ok_or_else(|| {
            "either --hours <HOURS_PER_DAY> or positional <HOURS_PER_DAY> is required".to_string()
        })?;
    if hours_per_day > 24 {
        return Err("hours_per_day must be between 0 and 24".to_string());
    }

    let days_per_week = parsed
        .days_per_week
        .or(parsed.positional_days_per_week)
        .unwrap_or(Decimal::from(5));
    validate_positive_decimal(days_per_week, "days_per_week")?;

    let weeks_per_year = parsed
        .weeks_per_year
        .or(parsed.positional_weeks_per_year)
        .unwrap_or(Decimal::from(52));
    validate_positive_decimal(weeks_per_year, "weeks_per_year")?;

    let months_per_year = parsed
        .months_per_year
        .or(parsed.positional_months_per_year)
        .unwrap_or(Decimal::from(12));
    validate_positive_decimal(months_per_year, "months_per_year")?;

    let inputs = rates
        .into_iter()
        .map(|rate| PayInput {
            rate,
            hours_per_day,
        })
        .collect();

    Ok(CliAction::Calculate {
        inputs,
        schedule: WorkSchedule {
            days_per_week,
            weeks_per_year,
            months_per_year,
        },
        format: parsed.format,
    })
}

fn parse_rates(value: &str) -> Result<Vec<Decimal>, String> {
    let parts: Vec<&str> = value.split(',').map(str::trim).collect();
    if parts.is_empty() || parts.iter().any(|part| part.is_empty()) {
        return Err("rate must be a comma-separated list of numbers".to_string());
    }

    let mut rates = Vec::with_capacity(parts.len());
    for part in parts {
        let rate: Decimal = part
            .parse()
            .map_err(|_| format!("invalid rate '{part}' in comma-separated rate list"))?;
        if rate < Decimal::ZERO {
            return Err("rate must be non-negative".to_string());
        }
        rates.push(rate);
    }

    Ok(rates)
}

fn validate_positive_decimal(value: Decimal, name: &str) -> Result<(), String> {
    if value <= Decimal::ZERO {
        return Err(format!("{name} must be greater than 0"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::{parse_args, CliAction, OutputFormat};
    use crate::{PayInput, WorkSchedule};

    #[test]
    fn parse_help_flag() {
        let action = parse_args(["--help".to_string()]).unwrap();
        assert!(matches!(action, CliAction::Help));
    }

    #[test]
    fn parse_rejects_missing_args() {
        let err = parse_args(Vec::<String>::new()).unwrap_err();
        assert!(err
            .contains("either --rate <RATE[,RATE...]> or positional <RATE[,RATE...]> is required"));
    }

    #[test]
    fn parse_rejects_negative_rate() {
        let err = parse_args([
            "--rate".to_string(),
            "-1".to_string(),
            "--hours".to_string(),
            "8".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "rate must be non-negative");
    }

    #[test]
    fn parse_rejects_invalid_rate() {
        let err = parse_args([
            "--rate".to_string(),
            "abc".to_string(),
            "--hours".to_string(),
            "8".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "invalid rate 'abc' in comma-separated rate list");
    }

    #[test]
    fn parse_rejects_invalid_rate_list() {
        let err = parse_args([
            "--rate".to_string(),
            "20,,30".to_string(),
            "--hours".to_string(),
            "8".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "rate must be a comma-separated list of numbers");
    }

    #[test]
    fn parse_rejects_invalid_hours() {
        let err = parse_args([
            "--rate".to_string(),
            "20".to_string(),
            "--hours".to_string(),
            "abc".to_string(),
        ])
        .unwrap_err();
        assert!(err.contains("invalid value 'abc' for '--hours <HOURS_PER_DAY>'"));
    }

    #[test]
    fn parse_rejects_unrealistic_hours() {
        let err = parse_args([
            "--rate".to_string(),
            "20".to_string(),
            "--hours".to_string(),
            "25".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "hours_per_day must be between 0 and 24");
    }

    #[test]
    fn parse_accepts_named_args_with_defaults() {
        let action = parse_args([
            "--rate".to_string(),
            "20,25".to_string(),
            "--hours".to_string(),
            "8".to_string(),
        ])
        .unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                inputs: vec![
                    PayInput {
                        rate: dec!(20.0),
                        hours_per_day: 8,
                    },
                    PayInput {
                        rate: dec!(25.0),
                        hours_per_day: 8,
                    },
                ],
                schedule: WorkSchedule::default(),
                format: OutputFormat::Table,
            }
        );
    }

    #[test]
    fn parse_accepts_export_format() {
        let action = parse_args([
            "--rate".to_string(),
            "20,25".to_string(),
            "--hours".to_string(),
            "8".to_string(),
            "--format".to_string(),
            "csv".to_string(),
        ])
        .unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                inputs: vec![
                    PayInput {
                        rate: dec!(20.0),
                        hours_per_day: 8,
                    },
                    PayInput {
                        rate: dec!(25.0),
                        hours_per_day: 8,
                    },
                ],
                schedule: WorkSchedule::default(),
                format: OutputFormat::Csv,
            }
        );
    }

    #[test]
    fn parse_accepts_custom_schedule_flags() {
        let action = parse_args([
            "--rate".to_string(),
            "20,25".to_string(),
            "--hours".to_string(),
            "8".to_string(),
            "--days-per-week".to_string(),
            "4".to_string(),
            "--weeks-per-year".to_string(),
            "48".to_string(),
            "--months-per-year".to_string(),
            "12".to_string(),
        ])
        .unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                inputs: vec![
                    PayInput {
                        rate: dec!(20.0),
                        hours_per_day: 8,
                    },
                    PayInput {
                        rate: dec!(25.0),
                        hours_per_day: 8,
                    },
                ],
                schedule: WorkSchedule {
                    days_per_week: dec!(4.0),
                    weeks_per_year: dec!(48.0),
                    months_per_year: dec!(12.0),
                },
                format: OutputFormat::Table,
            }
        );
    }

    #[test]
    fn parse_accepts_positional_args_for_backwards_compatibility() {
        let action = parse_args([
            "20,25".to_string(),
            "8".to_string(),
            "4".to_string(),
            "48".to_string(),
            "12".to_string(),
        ])
        .unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                inputs: vec![
                    PayInput {
                        rate: dec!(20.0),
                        hours_per_day: 8,
                    },
                    PayInput {
                        rate: dec!(25.0),
                        hours_per_day: 8,
                    },
                ],
                schedule: WorkSchedule {
                    days_per_week: dec!(4.0),
                    weeks_per_year: dec!(48.0),
                    months_per_year: dec!(12.0),
                },
                format: OutputFormat::Table,
            }
        );
    }

    #[test]
    fn parse_rejects_invalid_days_per_week() {
        let err = parse_args([
            "--rate".to_string(),
            "20".to_string(),
            "--hours".to_string(),
            "8".to_string(),
            "--days-per-week".to_string(),
            "0".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "days_per_week must be greater than 0");
    }

    #[test]
    fn parse_rejects_invalid_weeks_per_year() {
        let err = parse_args([
            "--rate".to_string(),
            "20".to_string(),
            "--hours".to_string(),
            "8".to_string(),
            "--weeks-per-year".to_string(),
            "0".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "weeks_per_year must be greater than 0");
    }

    #[test]
    fn parse_rejects_invalid_months_per_year() {
        let err = parse_args([
            "--rate".to_string(),
            "20".to_string(),
            "--hours".to_string(),
            "8".to_string(),
            "--months-per-year".to_string(),
            "0".to_string(),
        ])
        .unwrap_err();
        assert_eq!(err, "months_per_year must be greater than 0");
    }

    #[test]
    fn parse_accepts_decimal_rate() {
        let action = parse_args([
            "--rate".to_string(),
            "22.75,30.50".to_string(),
            "--hours".to_string(),
            "7".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .unwrap();

        assert_eq!(
            action,
            CliAction::Calculate {
                inputs: vec![
                    PayInput {
                        rate: dec!(22.75),
                        hours_per_day: 7,
                    },
                    PayInput {
                        rate: dec!(30.50),
                        hours_per_day: 7,
                    },
                ],
                schedule: WorkSchedule::default(),
                format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn parse_rejects_missing_hours_with_named_args() {
        let err = parse_args(["--rate".to_string(), "20,25".to_string()]).unwrap_err();
        assert!(err
            .contains("either --hours <HOURS_PER_DAY> or positional <HOURS_PER_DAY> is required"));
    }
}
