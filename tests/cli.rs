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
    assert!(stdout.contains("--rate <RATE>"));
    assert!(stdout.contains("--hours <HOURS_PER_DAY>"));
}

#[test]
fn named_args_render_expected_table() {
    let output = run_paycal(&["--rate", "20", "--hours", "8"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("| Hourly          |     20.00 |"));
    assert!(stdout.contains("| Weekly          |    800.00 |"));
    assert!(stdout.contains("| Monthly         |   3466.67 |"));
    assert!(stdout.contains("| Yearly          |  41600.00 |"));
}

#[test]
fn positional_args_still_work() {
    let output = run_paycal(&["20", "8", "4", "48", "12"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("| Weekly          |    640.00 |"));
    assert!(stdout.contains("| Monthly         |   2560.00 |"));
    assert!(stdout.contains("| Yearly          |  30720.00 |"));
}

#[test]
fn invalid_input_shows_friendly_error() {
    let output = run_paycal(&["--rate", "-1", "--hours", "8"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("Error: rate must be non-negative"));
}
