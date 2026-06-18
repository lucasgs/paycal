use std::process::ExitCode;

use paycal::{read_args, CliAction, USAGE};

fn main() -> ExitCode {
    match read_args() {
        Ok(CliAction::Help) => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Ok(CliAction::Calculate { input, schedule }) => {
            let result = input.calculate_with_schedule(schedule);
            println!("* Results *");
            println!("Hourly:  {:.2}", result.hourly);
            println!("Weekly:  {:.2}", result.weekly);
            println!("Monthly: {:.2}", result.monthly);
            println!("Yearly:  {:.2}", result.yearly);
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("Error: {message}\n\n{USAGE}");
            ExitCode::from(1)
        }
    }
}
