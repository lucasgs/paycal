/// Input values used to calculate pay across common time periods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PayInput {
    pub rate: f64,
    pub hours_per_day: u8,
}

impl PayInput {
    /// Calculates hourly, weekly, monthly, and yearly pay using
    /// 5 work days per week, 52 weeks per year, and 12 months per year.
    pub fn calculate(self) -> PayBreakdown {
        let days_per_week = 5.0;
        let hours_per_week = f64::from(self.hours_per_day) * days_per_week;

        let weeks_per_year = 52.0;
        let months_per_year = 12.0;
        let weeks_per_month = weeks_per_year / months_per_year;

        let hourly = self.rate;
        let monthly = self.rate * hours_per_week * weeks_per_month;
        let weekly = monthly / weeks_per_month;
        let yearly = monthly * months_per_year;

        PayBreakdown {
            hourly,
            weekly,
            monthly,
            yearly,
        }
    }
}

/// Calculated pay values across common time periods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PayBreakdown {
    pub hourly: f64,
    pub weekly: f64,
    pub monthly: f64,
    pub yearly: f64,
}

#[cfg(test)]
mod tests {
    use super::PayInput;

    #[test]
    fn calculate_round_case() {
        let data = PayInput {
            rate: 10.0,
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 10.0);
        assert_eq!(res.weekly, 400.0);
        assert_eq!(res.monthly, 1733.3333333333333);
        assert_eq!(res.yearly, 20800.0);
    }

    #[test]
    fn calculate_complex_case() {
        let data = PayInput {
            rate: 39.5,
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 39.5);
        assert_eq!(res.weekly, 1580.0);
        assert_eq!(res.monthly, 6846.666666666666);
        assert_eq!(res.yearly, 82160.0);
    }

    #[test]
    fn calculate_zero_rate_case() {
        let data = PayInput {
            rate: 0.0,
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 0.0);
        assert_eq!(res.weekly, 0.0);
        assert_eq!(res.monthly, 0.0);
        assert_eq!(res.yearly, 0.0);
    }

    #[test]
    fn calculate_zero_hours_case() {
        let data = PayInput {
            rate: 10.0,
            hours_per_day: 0,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, 10.0);
        assert_eq!(res.weekly, 0.0);
        assert_eq!(res.monthly, 0.0);
        assert_eq!(res.yearly, 0.0);
    }
}
