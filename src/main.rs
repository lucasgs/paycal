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
            currency,
        }) => {
            match format {
                OutputFormat::Table => print_table(&inputs, schedule, currency.as_deref()),
                OutputFormat::Csv => print_csv(&inputs, schedule, currency.as_deref()),
                OutputFormat::Json => print_json(&inputs, schedule, currency.as_deref()),
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("Error: {message}\n\n{}", usage());
            ExitCode::from(1)
        }
    }
}

fn print_table(inputs: &[PayInput], schedule: WorkSchedule, currency: Option<&str>) {
    let headers = ["Rate", "Weekly", "Monthly", "Yearly"];
    let rows: Vec<[String; 4]> = inputs
        .iter()
        .map(|input| {
            let result = input.calculate_with_schedule(schedule);
            [
                format_money(input.rate, currency),
                format_money(result.weekly, currency),
                format_money(result.monthly, currency),
                format_money(result.yearly, currency),
            ]
        })
        .collect();

    let widths = column_widths(&headers, &rows);
    print_border(&widths);
    print_row(headers, &widths);
    print_border(&widths);

    for row in rows {
        print_row([&row[0], &row[1], &row[2], &row[3]], &widths);
    }

    print_border(&widths);
}

fn print_csv(inputs: &[PayInput], schedule: WorkSchedule, currency: Option<&str>) {
    if let Some(currency) = currency {
        println!("currency,{currency}");
    }
    println!("rate,weekly,monthly,yearly");

    for input in inputs {
        let result = input.calculate_with_schedule(schedule);
        println!(
            "{},{},{},{}",
            format_money(input.rate, currency),
            format_money(result.weekly, currency),
            format_money(result.monthly, currency),
            format_money(result.yearly, currency),
        );
    }
}

fn print_json(inputs: &[PayInput], schedule: WorkSchedule, currency: Option<&str>) {
    let rows: Vec<JsonRow> = inputs
        .iter()
        .map(|input| {
            let result = input.calculate_with_schedule(schedule);
            JsonRow::from(*input, result, currency)
        })
        .collect();

    let payload = JsonExport {
        schedule: JsonSchedule::from(schedule),
        currency: currency.map(str::to_string),
        results: rows,
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&payload).expect("json serialization should succeed")
    );
}

fn format_money(value: Decimal, currency: Option<&str>) -> String {
    let amount = format!("{:.2}", value.round_dp(2));
    match currency {
        Some(currency) if currency.chars().all(|c| c.is_ascii_alphabetic()) => {
            format!("{currency} {amount}")
        }
        Some(currency) => format!("{currency}{amount}"),
        None => amount,
    }
}

fn column_widths(headers: &[&str; 4], rows: &[[String; 4]]) -> [usize; 4] {
    let mut widths = [0; 4];
    for index in 0..4 {
        widths[index] = headers[index].len();
        for row in rows {
            widths[index] = widths[index].max(row[index].len());
        }
    }
    widths
}

fn print_border(widths: &[usize; 4]) {
    println!(
        "+-{}-+-{}-+-{}-+-{}-+",
        "-".repeat(widths[0]),
        "-".repeat(widths[1]),
        "-".repeat(widths[2]),
        "-".repeat(widths[3]),
    );
}

fn print_row(values: [&str; 4], widths: &[usize; 4]) {
    println!(
        "| {:>width0$} | {:>width1$} | {:>width2$} | {:>width3$} |",
        values[0],
        values[1],
        values[2],
        values[3],
        width0 = widths[0],
        width1 = widths[1],
        width2 = widths[2],
        width3 = widths[3],
    );
}

#[derive(Serialize)]
struct JsonExport {
    schedule: JsonSchedule,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
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
            days_per_week: format_money(schedule.days_per_week, None),
            weeks_per_year: format_money(schedule.weeks_per_year, None),
            months_per_year: format_money(schedule.months_per_year, None),
        }
    }
}

#[derive(Serialize)]
struct JsonRow {
    rate: String,
    weekly: String,
    monthly: String,
    yearly: String,
}

impl JsonRow {
    fn from(input: PayInput, result: PayBreakdown, currency: Option<&str>) -> Self {
        Self {
            rate: format_money(input.rate, currency),
            weekly: format_money(result.weekly, currency),
            monthly: format_money(result.monthly, currency),
            yearly: format_money(result.yearly, currency),
        }
    }
}
