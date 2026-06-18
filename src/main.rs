use std::process::ExitCode;

use paycal::{read_args, usage, CliAction, OutputFormat, PayBreakdown, PayInput, WorkSchedule};
use rust_decimal::Decimal;
use serde::Serialize;

fn main() -> ExitCode {
    match read_args() {
        Ok(CliAction::Help) => {
            println!("{}", usage());
            ExitCode::SUCCESS
        }
        Ok(CliAction::Calculate {
            inputs,
            schedule,
            format,
        }) => {
            match format {
                OutputFormat::Table => print_table(&inputs, schedule),
                OutputFormat::Csv => print_csv(&inputs, schedule),
                OutputFormat::Json => print_json(&inputs, schedule),
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("Error: {message}\n\n{}", usage());
            ExitCode::from(1)
        }
    }
}

fn print_table(inputs: &[PayInput], schedule: WorkSchedule) {
    println!("+----------+----------+----------+----------+----------+");
    println!("| Rate     | Hourly   | Weekly   | Monthly  | Yearly   |");
    println!("+----------+----------+----------+----------+----------+");

    for input in inputs {
        let result = input.calculate_with_schedule(schedule);
        println!(
            "| {:>8} | {:>8} | {:>8} | {:>8} | {:>8} |",
            format_money(input.rate),
            format_money(result.hourly),
            format_money(result.weekly),
            format_money(result.monthly),
            format_money(result.yearly),
        );
    }

    println!("+----------+----------+----------+----------+----------+");
}

fn print_csv(inputs: &[PayInput], schedule: WorkSchedule) {
    println!("rate,hourly,weekly,monthly,yearly");

    for input in inputs {
        let result = input.calculate_with_schedule(schedule);
        println!(
            "{},{},{},{},{}",
            format_money(input.rate),
            format_money(result.hourly),
            format_money(result.weekly),
            format_money(result.monthly),
            format_money(result.yearly),
        );
    }
}

fn print_json(inputs: &[PayInput], schedule: WorkSchedule) {
    let rows: Vec<JsonRow> = inputs
        .iter()
        .map(|input| {
            let result = input.calculate_with_schedule(schedule);
            JsonRow::from(*input, result)
        })
        .collect();

    let payload = JsonExport {
        schedule: JsonSchedule::from(schedule),
        results: rows,
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&payload).expect("json serialization should succeed")
    );
}

fn format_money(value: Decimal) -> String {
    format!("{:.2}", value.round_dp(2))
}

#[derive(Serialize)]
struct JsonExport {
    schedule: JsonSchedule,
    results: Vec<JsonRow>,
}

#[derive(Serialize)]
struct JsonSchedule {
    days_per_week: String,
    weeks_per_year: String,
    months_per_year: String,
}

impl From<WorkSchedule> for JsonSchedule {
    fn from(schedule: WorkSchedule) -> Self {
        Self {
            days_per_week: format_money(schedule.days_per_week),
            weeks_per_year: format_money(schedule.weeks_per_year),
            months_per_year: format_money(schedule.months_per_year),
        }
    }
}

#[derive(Serialize)]
struct JsonRow {
    rate: String,
    hourly: String,
    weekly: String,
    monthly: String,
    yearly: String,
}

impl JsonRow {
    fn from(input: PayInput, result: PayBreakdown) -> Self {
        Self {
            rate: format_money(input.rate),
            hourly: format_money(result.hourly),
            weekly: format_money(result.weekly),
            monthly: format_money(result.monthly),
            yearly: format_money(result.yearly),
        }
    }
}
