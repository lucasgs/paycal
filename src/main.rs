use std::{cmp::Ordering, fs, process::ExitCode};

use paycal::{
    read_args, usage, CliAction, OutputFormat, PayBreakdown, PayInput, SortBy, WorkSchedule,
};
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
            output,
            sort,
        }) => {
            let rendered = match format {
                OutputFormat::Table => render_table(&inputs, schedule, currency.as_deref(), sort),
                OutputFormat::Csv => render_csv(&inputs, schedule, currency.as_deref(), sort),
                OutputFormat::Json => render_json(&inputs, schedule, currency.as_deref(), sort),
            };

            match output_result(output.as_deref(), &rendered) {
                Ok(()) => ExitCode::SUCCESS,
                Err(message) => {
                    eprintln!("Error: {message}");
                    ExitCode::from(1)
                }
            }
        }
        Err(message) => {
            eprintln!("Error: {message}\n\n{}", usage());
            ExitCode::from(1)
        }
    }
}

fn output_result(path: Option<&str>, content: &str) -> Result<(), String> {
    match path {
        Some(path) => fs::write(path, content)
            .map_err(|error| format!("failed to write output file '{path}': {error}")),
        None => {
            print!("{content}");
            Ok(())
        }
    }
}

fn render_table(
    inputs: &[PayInput],
    schedule: WorkSchedule,
    currency: Option<&str>,
    sort: Option<SortBy>,
) -> String {
    let headers = ["Rate", "Weekly", "Monthly", "Yearly"];
    let sorted_inputs = sorted_inputs(inputs, schedule, sort);
    let rows: Vec<[String; 4]> = sorted_inputs
        .iter()
        .map(|(input, result)| {
            [
                format_money(input.rate, currency),
                format_money(result.weekly, currency),
                format_money(result.monthly, currency),
                format_money(result.yearly, currency),
            ]
        })
        .collect();

    let widths = column_widths(&headers, &rows);
    let mut output = String::new();
    push_border(&mut output, &widths);
    push_row(&mut output, headers, &widths);
    push_border(&mut output, &widths);

    for row in rows {
        push_row(&mut output, [&row[0], &row[1], &row[2], &row[3]], &widths);
    }

    push_border(&mut output, &widths);
    output
}

fn render_csv(
    inputs: &[PayInput],
    schedule: WorkSchedule,
    currency: Option<&str>,
    sort: Option<SortBy>,
) -> String {
    let mut output = String::new();
    let sorted_inputs = sorted_inputs(inputs, schedule, sort);

    if let Some(currency) = currency {
        output.push_str(&format!("currency,{currency}\n"));
    }
    if let Some(sort) = sort {
        output.push_str(&format!("sort,{}\n", sort_label(sort)));
    }
    output.push_str("rate,weekly,monthly,yearly\n");

    for (input, result) in sorted_inputs {
        output.push_str(&format!(
            "{},{},{},{}\n",
            format_money(input.rate, currency),
            format_money(result.weekly, currency),
            format_money(result.monthly, currency),
            format_money(result.yearly, currency),
        ));
    }

    output
}

fn render_json(
    inputs: &[PayInput],
    schedule: WorkSchedule,
    currency: Option<&str>,
    sort: Option<SortBy>,
) -> String {
    let sorted_inputs = sorted_inputs(inputs, schedule, sort);
    let rows: Vec<JsonRow> = sorted_inputs
        .into_iter()
        .map(|(input, result)| JsonRow::from(input, result, currency))
        .collect();

    let payload = JsonExport {
        schedule: JsonSchedule::from(schedule),
        currency: currency.map(str::to_string),
        sort: sort.map(sort_label),
        results: rows,
    };

    format!(
        "{}\n",
        serde_json::to_string_pretty(&payload).expect("json serialization should succeed")
    )
}

fn sorted_inputs(
    inputs: &[PayInput],
    schedule: WorkSchedule,
    sort: Option<SortBy>,
) -> Vec<(PayInput, PayBreakdown)> {
    let mut rows: Vec<(PayInput, PayBreakdown)> = inputs
        .iter()
        .copied()
        .map(|input| (input, input.calculate_with_schedule(schedule)))
        .collect();

    if let Some(sort) = sort {
        rows.sort_by(|left, right| compare_rows(left, right, sort));
    }

    rows
}

fn compare_rows(
    left: &(PayInput, PayBreakdown),
    right: &(PayInput, PayBreakdown),
    sort: SortBy,
) -> Ordering {
    let primary = match sort {
        SortBy::Rate => left.0.rate.cmp(&right.0.rate),
        SortBy::Weekly => left.1.weekly.cmp(&right.1.weekly),
        SortBy::Monthly => left.1.monthly.cmp(&right.1.monthly),
        SortBy::Yearly => left.1.yearly.cmp(&right.1.yearly),
    };

    primary.then_with(|| left.0.rate.cmp(&right.0.rate))
}

fn sort_label(sort: SortBy) -> String {
    match sort {
        SortBy::Rate => "rate",
        SortBy::Weekly => "weekly",
        SortBy::Monthly => "monthly",
        SortBy::Yearly => "yearly",
    }
    .to_string()
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

fn push_border(output: &mut String, widths: &[usize; 4]) {
    output.push_str(&format!(
        "+-{}-+-{}-+-{}-+-{}-+\n",
        "-".repeat(widths[0]),
        "-".repeat(widths[1]),
        "-".repeat(widths[2]),
        "-".repeat(widths[3]),
    ));
}

fn push_row(output: &mut String, values: [&str; 4], widths: &[usize; 4]) {
    output.push_str(&format!(
        "| {:>width0$} | {:>width1$} | {:>width2$} | {:>width3$} |\n",
        values[0],
        values[1],
        values[2],
        values[3],
        width0 = widths[0],
        width1 = widths[1],
        width2 = widths[2],
        width3 = widths[3],
    ));
}

#[derive(Serialize)]
struct JsonExport {
    schedule: JsonSchedule,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
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
