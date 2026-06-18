use std::process::ExitCode;

use paycal::{read_args, usage, CliAction, WorkSchedule};
use rust_decimal::Decimal;

fn main() -> ExitCode {
    match read_args() {
        Ok(CliAction::Help) => {
            println!("{}", usage());
            ExitCode::SUCCESS
        }
        Ok(CliAction::Calculate { inputs, schedule }) => {
            print_results_table(&inputs, schedule);
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("Error: {message}\n\n{}", usage());
            ExitCode::from(1)
        }
    }
}

fn print_results_table(inputs: &[paycal::PayInput], schedule: WorkSchedule) {
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

fn format_money(value: Decimal) -> String {
    format!("{:.2}", value.round_dp(2))
}
