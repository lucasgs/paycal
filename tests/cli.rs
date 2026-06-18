use std::process::Command;

fn run_paycal(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_paycal"))
        .args(args)
        .output()
        .expect("failed to run paycal")
}

#[test]
fn help_output_succeeds() {
    let output = run_paycal(&["--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Usage: paycal [OPTIONS]"));
    assert!(stdout.contains("--rate <RATE[,RATE...]>"));
    assert!(stdout.contains("--hours <HOURS_PER_DAY>"));
}

#[test]
fn named_args_render_expected_table() {
    let output = run_paycal(&["--rate", "20", "--hours", "8"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("| Rate     | Hourly   | Weekly   | Monthly  | Yearly   |"));
    assert!(stdout.contains("|    20.00 |    20.00 |   800.00 |  3466.67 | 41600.00 |"));
}

#[test]
fn multi_rate_named_args_render_comparison_table() {
    let output = run_paycal(&["--rate", "20,25", "--hours", "8"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("|    20.00 |    20.00 |   800.00 |  3466.67 | 41600.00 |"));
    assert!(stdout.contains("|    25.00 |    25.00 |  1000.00 |  4333.33 | 52000.00 |"));
}

#[test]
fn positional_args_still_work() {
    let output = run_paycal(&["20,25", "8", "4", "48", "12"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("|    20.00 |    20.00 |   640.00 |  2560.00 | 30720.00 |"));
    assert!(stdout.contains("|    25.00 |    25.00 |   800.00 |  3200.00 | 38400.00 |"));
}

#[test]
fn invalid_input_shows_friendly_error() {
    let output = run_paycal(&["--rate", "-1", "--hours", "8"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("Error: rate must be non-negative"));
}
