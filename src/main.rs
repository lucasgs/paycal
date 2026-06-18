use std::{
    cmp::Ordering,
    env, fs,
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

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
            locale,
            output,
            sort,
        }) => {
            let generated_at_unix_seconds = current_unix_timestamp();
            let locale = resolve_locale(locale.as_deref());
            let rendered = match format {
                OutputFormat::Table => {
                    render_table(&inputs, schedule, currency.as_deref(), sort, &locale)
                }
                OutputFormat::Csv => render_csv(
                    &inputs,
                    schedule,
                    currency.as_deref(),
                    sort,
                    generated_at_unix_seconds,
                    &locale,
                ),
                OutputFormat::Json => render_json(
                    &inputs,
                    schedule,
                    currency.as_deref(),
                    sort,
                    generated_at_unix_seconds,
                    &locale,
                ),
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
    locale: &NumberLocale,
) -> String {
    let headers = ["Rate", "Weekly", "Monthly", "Yearly"];
    let sorted_inputs = sorted_inputs(inputs, schedule, sort);
    let rows: Vec<[String; 4]> = sorted_inputs
        .iter()
        .map(|(input, result)| {
            [
                format_money(input.rate, currency, locale),
                format_money(result.weekly, currency, locale),
                format_money(result.monthly, currency, locale),
                format_money(result.yearly, currency, locale),
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
    generated_at_unix_seconds: u64,
    locale: &NumberLocale,
) -> String {
    let mut output = String::new();
    let sorted_inputs = sorted_inputs(inputs, schedule, sort);
    let metadata = ExportMetadata::new(
        inputs,
        OutputFormat::Csv,
        currency,
        sort,
        generated_at_unix_seconds,
        locale,
    );

    output.push_str(&csv_record(&["format", format_label(metadata.format)]));
    output.push_str(&csv_record(&["locale", &metadata.locale_code]));
    output.push_str(&csv_record(&[
        "hours_per_day",
        &metadata.hours_per_day.to_string(),
    ]));
    output.push_str(&csv_record(&[
        "days_per_week",
        &format_number(schedule.days_per_week, locale),
    ]));
    output.push_str(&csv_record(&[
        "weeks_per_year",
        &format_number(schedule.weeks_per_year, locale),
    ]));
    output.push_str(&csv_record(&[
        "months_per_year",
        &format_number(schedule.months_per_year, locale),
    ]));
    output.push_str(&csv_record(&[
        "generated_at_unix_seconds",
        &metadata.generated_at_unix_seconds.to_string(),
    ]));
    if let Some(currency) = metadata.currency {
        output.push_str(&csv_record(&["currency", currency]));
    }
    if let Some(sort) = metadata.sort {
        output.push_str(&csv_record(&["sort", sort_label(sort)]));
    }
    output.push_str("rate,weekly,monthly,yearly\n");

    for (input, result) in sorted_inputs {
        output.push_str(&csv_record(&[
            &format_money(input.rate, currency, locale),
            &format_money(result.weekly, currency, locale),
            &format_money(result.monthly, currency, locale),
            &format_money(result.yearly, currency, locale),
        ]));
    }

    output
}

fn render_json(
    inputs: &[PayInput],
    schedule: WorkSchedule,
    currency: Option<&str>,
    sort: Option<SortBy>,
    generated_at_unix_seconds: u64,
    locale: &NumberLocale,
) -> String {
    let sorted_inputs = sorted_inputs(inputs, schedule, sort);
    let rows: Vec<JsonRow> = sorted_inputs
        .into_iter()
        .map(|(input, result)| JsonRow::from(input, result, currency, locale))
        .collect();
    let metadata = ExportMetadata::new(
        inputs,
        OutputFormat::Json,
        currency,
        sort,
        generated_at_unix_seconds,
        locale,
    );

    let payload = JsonExport {
        metadata: JsonMetadata::from(metadata),
        schedule: JsonSchedule::from(schedule, locale),
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

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_secs()
}

fn sort_label(sort: SortBy) -> &'static str {
    match sort {
        SortBy::Rate => "rate",
        SortBy::Weekly => "weekly",
        SortBy::Monthly => "monthly",
        SortBy::Yearly => "yearly",
    }
}

fn format_label(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::Table => "table",
        OutputFormat::Csv => "csv",
        OutputFormat::Json => "json",
    }
}

fn resolve_locale(cli_locale: Option<&str>) -> NumberLocale {
    let code = cli_locale
        .filter(|value| !value.trim().is_empty())
        .map(str::trim)
        .map(str::to_string)
        .or_else(detect_env_locale)
        .unwrap_or_else(|| "en-US".to_string());

    NumberLocale::from_code(&code)
}

fn detect_env_locale() -> Option<String> {
    ["LC_ALL", "LC_NUMERIC", "LANG"]
        .into_iter()
        .find_map(|key| env::var(key).ok())
        .and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
}

fn format_money(value: Decimal, currency: Option<&str>, locale: &NumberLocale) -> String {
    let amount = format_number(value, locale);
    match currency {
        Some(currency) if currency.chars().all(|c| c.is_ascii_alphabetic()) => {
            format!("{currency} {amount}")
        }
        Some(currency) => format!("{currency}{amount}"),
        None => amount,
    }
}

fn format_number(value: Decimal, locale: &NumberLocale) -> String {
    let rounded = value.round_dp(2);
    let raw = format!("{rounded:.2}");
    let (negative, digits) = raw
        .strip_prefix('-')
        .map_or((false, raw.as_str()), |rest| (true, rest));
    let (integer, fraction) = digits
        .split_once('.')
        .expect("formatted decimals always contain a dot");
    let grouped = group_digits(integer, locale.grouping_separator);
    let sign = if negative { "-" } else { "" };
    format!("{sign}{grouped}{}{fraction}", locale.decimal_separator)
}

fn group_digits(integer: &str, separator: char) -> String {
    let mut reversed = String::with_capacity(integer.len() + integer.len() / 3);
    for (index, ch) in integer.chars().rev().enumerate() {
        if index != 0 && index % 3 == 0 {
            reversed.push(separator);
        }
        reversed.push(ch);
    }
    reversed.chars().rev().collect()
}

fn csv_record(values: &[&str]) -> String {
    let escaped = values
        .iter()
        .map(|value| escape_csv(value))
        .collect::<Vec<_>>();
    format!("{}\n", escaped.join(","))
}

fn escape_csv(value: &str) -> String {
    if value.contains([',', '"', '\n']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
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

#[derive(Debug, Clone)]
struct NumberLocale {
    code: String,
    decimal_separator: char,
    grouping_separator: char,
}

impl NumberLocale {
    fn from_code(code: &str) -> Self {
        let code = canonicalize_locale_code(code);
        let normalized = code.to_ascii_lowercase();

        let (decimal_separator, grouping_separator) =
            if normalized.starts_with("fr") || normalized.starts_with("ru") {
                (',', ' ')
            } else if normalized.starts_with("de")
                || normalized.starts_with("es")
                || normalized.starts_with("it")
                || normalized.starts_with("pt")
                || normalized.starts_with("nl")
                || normalized.starts_with("tr")
            {
                (',', '.')
            } else {
                ('.', ',')
            };

        Self {
            code,
            decimal_separator,
            grouping_separator,
        }
    }
}

fn canonicalize_locale_code(code: &str) -> String {
    let base = code.split('.').next().unwrap_or(code).replace('_', "-");
    let mut parts = base.split('-');
    let language = parts.next().unwrap_or_default().to_ascii_lowercase();

    match parts.next() {
        Some(region) if !language.is_empty() => {
            format!("{}-{}", language, region.to_ascii_uppercase())
        }
        _ if !language.is_empty() => language,
        _ => "en-US".to_string(),
    }
}

#[derive(Debug, Clone)]
struct ExportMetadata<'a> {
    format: OutputFormat,
    locale_code: String,
    hours_per_day: u8,
    currency: Option<&'a str>,
    sort: Option<SortBy>,
    generated_at_unix_seconds: u64,
}

impl<'a> ExportMetadata<'a> {
    fn new(
        inputs: &[PayInput],
        format: OutputFormat,
        currency: Option<&'a str>,
        sort: Option<SortBy>,
        generated_at_unix_seconds: u64,
        locale: &NumberLocale,
    ) -> Self {
        Self {
            format,
            locale_code: locale.code.clone(),
            hours_per_day: inputs.first().map_or(0, |input| input.hours_per_day),
            currency,
            sort,
            generated_at_unix_seconds,
        }
    }
}

#[derive(Serialize)]
struct JsonExport {
    metadata: JsonMetadata,
    schedule: JsonSchedule,
    results: Vec<JsonRow>,
}

#[derive(Serialize)]
struct JsonMetadata {
    format: String,
    locale: String,
    hours_per_day: u8,
    generated_at_unix_seconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
}

impl<'a> From<ExportMetadata<'a>> for JsonMetadata {
    fn from(metadata: ExportMetadata<'a>) -> Self {
        Self {
            format: format_label(metadata.format).to_string(),
            locale: metadata.locale_code,
            hours_per_day: metadata.hours_per_day,
            generated_at_unix_seconds: metadata.generated_at_unix_seconds,
            currency: metadata.currency.map(str::to_string),
            sort: metadata.sort.map(|sort| sort_label(sort).to_string()),
        }
    }
}

#[derive(Serialize)]
struct JsonSchedule {
    days_per_week: String,
    weeks_per_year: String,
    months_per_year: String,
}

impl JsonSchedule {
    fn from(schedule: WorkSchedule, locale: &NumberLocale) -> Self {
        Self {
            days_per_week: format_number(schedule.days_per_week, locale),
            weeks_per_year: format_number(schedule.weeks_per_year, locale),
            months_per_year: format_number(schedule.months_per_year, locale),
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
    fn from(
        input: PayInput,
        result: PayBreakdown,
        currency: Option<&str>,
        locale: &NumberLocale,
    ) -> Self {
        Self {
            rate: format_money(input.rate, currency, locale),
            weekly: format_money(result.weekly, currency, locale),
            monthly: format_money(result.monthly, currency, locale),
            yearly: format_money(result.yearly, currency, locale),
        }
    }
}
