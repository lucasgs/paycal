use std::process::ExitCode;

use paycal::{read_args, usage, CliAction};
use rust_decimal::Decimal;

fn main() -> ExitCode {
    match read_args() {
        Ok(CliAction::Help) => {
            println!("{}", usage());
            ExitCode::SUCCESS
        }
        Ok(CliAction::Calculate { input, schedule }) => {
            let result = input.calculate_with_schedule(schedule);
            println!("+-----------------+-----------+");
            println!("| Period          | Amount    |");
            println!("+-----------------+-----------+");
            println!("| Hourly          | {:>9} |", format_money(result.hourly));
            println!("| Weekly          | {:>9} |", format_money(result.weekly));
            println!("| Monthly         | {:>9} |", format_money(result.monthly));
            println!("| Yearly          | {:>9} |", format_money(result.yearly));
            println!("+-----------------+-----------+");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("Error: {message}\n\n{}", usage());
            ExitCode::from(1)
        }
    }
}

fn format_money(value: Decimal) -> String {
    format!("{:.2}", value.round_dp(2))
}
