use std::{env, process::ExitCode};

#[cfg(test)]
mod tests;

const USAGE: &str = "paycal - CLI pay calculator\n\nUsage:\n  paycal <rate> <hours_per_day>\n  paycal --help\n\nArguments:\n  <rate>           Hourly pay rate (must be non-negative)\n  <hours_per_day>  Hours worked per day (must be between 0 and 24)\n\nExamples:\n  paycal 20 8\n  cargo run -- 20 8";

#[derive(Debug, Clone, Copy)]
struct Data {
    rate: f64,
    hours_per_day: u8,
}

impl Data {
    fn calculate(self) -> PayResult {
        let days_per_week = 5.0;
        let hours_per_week = f64::from(self.hours_per_day) * days_per_week;

        let weeks_per_year = 52.0;
        let months_per_year = 12.0;
        let weeks_per_month = weeks_per_year / months_per_year;

        let hourly = self.rate;
        let monthly = self.rate * hours_per_week * weeks_per_month;
        let weekly = monthly / weeks_per_month;
        let yearly = monthly * months_per_year;

        PayResult {
            hourly,
            weekly,
            monthly,
            yearly,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PayResult {
    hourly: f64,
    weekly: f64,
    monthly: f64,
    yearly: f64,
}

fn main() -> ExitCode {
    match read_args() {
        Ok(CliAction::Help) => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Ok(CliAction::Calculate(data)) => {
            let result = data.calculate();
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

#[derive(Debug)]
enum CliAction {
    Help,
    Calculate(Data),
}

fn read_args() -> std::result::Result<CliAction, String> {
    parse_args(env::args().skip(1))
}

fn parse_args<I>(args: I) -> std::result::Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();

    if args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h") {
        return Ok(CliAction::Help);
    }

    if args.len() != 2 {
        return Err("expected exactly 2 arguments: <rate> <hours_per_day>".to_string());
    }

    let rate: f64 = args[0]
        .parse()
        .map_err(|_| "rate must be a valid number".to_string())?;
    if !rate.is_finite() {
        return Err("rate must be a finite number".to_string());
    }
    if rate < 0.0 {
        return Err("rate must be non-negative".to_string());
    }

    let hours_per_day: u8 = args[1]
        .parse()
        .map_err(|_| "hours_per_day must be a whole number between 0 and 24".to_string())?;
    if hours_per_day > 24 {
        return Err("hours_per_day must be between 0 and 24".to_string());
    }

    Ok(CliAction::Calculate(Data {
        rate,
        hours_per_day,
    }))
}
