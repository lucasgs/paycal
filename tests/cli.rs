use std::{
    env, fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn run_paycal(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_paycal"))
        .args(args)
        .output()
        .expect("failed to run paycal")
}

fn temp_output_path(extension: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should move forward")
        .as_nanos();
    env::temp_dir()
        .join(format!("paycal_test_{timestamp}.{extension}"))
        .to_string_lossy()
        .into_owned()
}

#[test]
fn help_output_succeeds() {
    let output = run_paycal(&["--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Usage: paycal [OPTIONS]"));
    assert!(
        stdout.contains("--rate <RATE[,RATE...] >") || stdout.contains("--rate <RATE[,RATE...]>")
    );
    assert!(stdout.contains("--hours <HOURS_PER_DAY>"));
    assert!(stdout.contains("--format <FORMAT>"));
    assert!(stdout.contains("--currency <CURRENCY>"));
    assert!(stdout.contains("--output <FILE>"));
    assert!(stdout.contains("--sort <FIELD>"));
}

#[test]
fn named_args_render_expected_table() {
    let output = run_paycal(&["--rate", "20", "--hours", "8"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Rate"));
    assert!(stdout.contains("Weekly"));
    assert!(stdout.contains("Monthly"));
    assert!(stdout.contains("Yearly"));
    assert!(stdout.contains("| 20.00 | 800.00 | 3466.67 | 41600.00 |"));
    assert!(!stdout.contains("Hourly"));
}

#[test]
fn sorting_by_yearly_reorders_rows() {
    let output = run_paycal(&["--rate", "25,20,30", "--hours", "8", "--sort", "yearly"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let twenty = stdout.find("20.00 |  800.00 | 3466.67 | 41600.00").unwrap();
    let twenty_five = stdout.find("25.00 | 1000.00 | 4333.33 | 52000.00").unwrap();
    let thirty = stdout.find("30.00 | 1200.00 | 5200.00 | 62400.00").unwrap();
    assert!(twenty < twenty_five && twenty_five < thirty);
}

#[test]
fn currency_code_formats_all_output_columns() {
    let output = run_paycal(&["--rate", "20,25", "--hours", "8", "--currency", "USD"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Rate"));
    assert!(stdout.contains("Weekly"));
    assert!(stdout.contains("Monthly"));
    assert!(stdout.contains("Yearly"));
    assert!(stdout.contains("| USD 20.00 |  USD 800.00 | USD 3466.67 | USD 41600.00 |"));
    assert!(stdout.contains("| USD 25.00 | USD 1000.00 | USD 4333.33 | USD 52000.00 |"));
}

#[test]
fn currency_symbol_formats_all_output_columns() {
    let output = run_paycal(&["--rate", "20", "--hours", "8", "--currency", "$"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Rate"));
    assert!(stdout.contains("Weekly"));
    assert!(stdout.contains("Monthly"));
    assert!(stdout.contains("Yearly"));
    assert!(stdout.contains("| $20.00 | $800.00 | $3466.67 | $41600.00 |"));
}

#[test]
fn csv_export_works() {
    let output = run_paycal(&["--rate", "20,25", "--hours", "8", "--format", "csv"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("rate,weekly,monthly,yearly"));
    assert!(stdout.contains("20.00,800.00,3466.67,41600.00"));
    assert!(stdout.contains("25.00,1000.00,4333.33,52000.00"));
}

#[test]
fn csv_export_includes_sort_metadata() {
    let output = run_paycal(&[
        "--rate", "25,20", "--hours", "8", "--format", "csv", "--sort", "rate",
    ]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("sort,rate"));
    let twenty = stdout.find("20.00,800.00,3466.67,41600.00").unwrap();
    let twenty_five = stdout.find("25.00,1000.00,4333.33,52000.00").unwrap();
    assert!(twenty < twenty_five);
}

#[test]
fn csv_export_with_currency_works() {
    let output = run_paycal(&[
        "--rate",
        "20,25",
        "--hours",
        "8",
        "--format",
        "csv",
        "--currency",
        "USD",
    ]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("currency,USD"));
    assert!(stdout.contains("USD 20.00,USD 800.00,USD 3466.67,USD 41600.00"));
}

#[test]
fn json_export_works() {
    let output = run_paycal(&["--rate", "20,25", "--hours", "8", "--format", "json"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("\"schedule\""));
    assert!(stdout.contains("\"results\""));
    assert!(stdout.contains("\"rate\": \"20.00\""));
    assert!(stdout.contains("\"yearly\": \"52000.00\""));
    assert!(!stdout.contains("\"hourly\""));
}

#[test]
fn json_export_includes_sort_metadata() {
    let output = run_paycal(&[
        "--rate", "25,20", "--hours", "8", "--format", "json", "--sort", "yearly",
    ]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("\"sort\": \"yearly\""));
    let twenty = stdout.find("\"rate\": \"20.00\"").unwrap();
    let twenty_five = stdout.find("\"rate\": \"25.00\"").unwrap();
    assert!(twenty < twenty_five);
}

#[test]
fn json_export_with_currency_works() {
    let output = run_paycal(&[
        "--rate",
        "20,25",
        "--hours",
        "8",
        "--format",
        "json",
        "--currency",
        "USD",
    ]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("\"currency\": \"USD\""));
    assert!(stdout.contains("\"rate\": \"USD 20.00\""));
    assert!(stdout.contains("\"yearly\": \"USD 52000.00\""));
}

#[test]
fn output_file_writes_csv_and_suppresses_stdout() {
    let path = temp_output_path("csv");
    let output = run_paycal(&[
        "--rate", "20,25", "--hours", "8", "--format", "csv", "--output", &path,
    ]);

    assert!(output.status.success());
    assert!(output.stdout.is_empty());

    let written = fs::read_to_string(&path).expect("output file should exist");
    assert!(written.contains("rate,weekly,monthly,yearly"));
    assert!(written.contains("20.00,800.00,3466.67,41600.00"));

    let _ = fs::remove_file(path);
}

#[test]
fn positional_args_still_work() {
    let output = run_paycal(&["20,25", "8", "4", "48", "12"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("| 20.00 | 640.00 | 2560.00 | 30720.00 |"));
    assert!(stdout.contains("| 25.00 | 800.00 | 3200.00 | 38400.00 |"));
}

#[test]
fn invalid_input_shows_friendly_error() {
    let output = run_paycal(&["--rate", "-1", "--hours", "8"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("Error: rate must be non-negative"));
}
