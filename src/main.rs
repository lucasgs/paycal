use std::env;

mod tests;

#[derive(Debug, Clone, Copy)]
struct Data {
    rate: f64,
    hours_per_day: u8,
}

impl Data {
    fn calculate(self) -> Result {
        let days_per_week = 5;
        let hours_per_week = self.hours_per_day as f64 * days_per_week as f64;

        let weeks_per_year = 52;
        let months_per_year = 12;
        let weeks_per_month = weeks_per_year as f64 / months_per_year as f64;

        let hourly = self.rate;
        let monthly = self.rate * hours_per_week * weeks_per_month;
        let weekly = monthly / weeks_per_month as f64;
        let yearly = monthly * months_per_year as f64;

        Result {
            hourly,
            weekly,
            monthly,
            yearly,
        }
    }
}

struct Result {
    hourly: f64,
    weekly: f64,
    monthly: f64,
    yearly: f64,
}

fn main() {
    let data = read_args();
    // println!("{:?}", data);

    let result = data.calculate();
    println!("* Results *");
    println!("Hourly:  {}", result.hourly.floor());
    println!("Weekly:  {}", result.weekly.floor());
    println!("Monthly: {}", result.monthly.floor());
    println!("Yearly:  {}", result.yearly.floor());
}

fn read_args() -> Data {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("incorrect args should be 2: [rate] [hours_per_day]");
    }
    Data {
        rate: args[0].parse().expect("Invalid param rate"),
        hours_per_day: args[1].parse().expect("Invalid param hours_per_day"),
    }
}
