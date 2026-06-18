use rust_decimal::Decimal;

/// Input values used to calculate pay across common time periods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PayInput {
    pub rate: Decimal,
    pub hours_per_day: u8,
}

/// Configurable schedule assumptions used in pay calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorkSchedule {
    pub days_per_week: Decimal,
    pub weeks_per_year: Decimal,
    pub months_per_year: Decimal,
}

impl Default for WorkSchedule {
    fn default() -> Self {
        Self {
            days_per_week: Decimal::from(5),
            weeks_per_year: Decimal::from(52),
            months_per_year: Decimal::from(12),
        }
    }
}

impl PayInput {
    /// Calculates hourly, weekly, monthly, and yearly pay using the default
    /// schedule assumptions of 5 work days per week, 52 weeks per year,
    /// and 12 months per year.
    pub fn calculate(self) -> PayBreakdown {
        self.calculate_with_schedule(WorkSchedule::default())
    }

    /// Calculates hourly, weekly, monthly, and yearly pay using a caller-
    /// provided work schedule.
    pub fn calculate_with_schedule(self, schedule: WorkSchedule) -> PayBreakdown {
        let hours_per_week = Decimal::from(self.hours_per_day) * schedule.days_per_week;

        let hourly = self.rate;
        let weekly = self.rate * hours_per_week;
        let yearly = weekly * schedule.weeks_per_year;
        let monthly = yearly / schedule.months_per_year;

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
    pub hourly: Decimal,
    pub weekly: Decimal,
    pub monthly: Decimal,
    pub yearly: Decimal,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::{PayInput, WorkSchedule};

    #[test]
    fn calculate_round_case() {
        let data = PayInput {
            rate: dec!(10.0),
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, dec!(10.0));
        assert_eq!(res.weekly, dec!(400.0));
        assert_eq!(res.monthly.round_dp(2), dec!(1733.33));
        assert_eq!(res.yearly, dec!(20800.0));
    }

    #[test]
    fn calculate_complex_case() {
        let data = PayInput {
            rate: dec!(39.5),
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, dec!(39.5));
        assert_eq!(res.weekly, dec!(1580.0));
        assert_eq!(res.monthly.round_dp(2), dec!(6846.67));
        assert_eq!(res.yearly, dec!(82160.0));
    }

    #[test]
    fn calculate_zero_rate_case() {
        let data = PayInput {
            rate: dec!(0.0),
            hours_per_day: 8,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, dec!(0.0));
        assert_eq!(res.weekly, dec!(0.0));
        assert_eq!(res.monthly, dec!(0.0));
        assert_eq!(res.yearly, dec!(0.0));
    }

    #[test]
    fn calculate_zero_hours_case() {
        let data = PayInput {
            rate: dec!(10.0),
            hours_per_day: 0,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, dec!(10.0));
        assert_eq!(res.weekly, dec!(0.0));
        assert_eq!(res.monthly, dec!(0.0));
        assert_eq!(res.yearly, dec!(0.0));
    }

    #[test]
    fn calculate_with_custom_schedule() {
        let data = PayInput {
            rate: dec!(20.0),
            hours_per_day: 8,
        };

        let res = data.calculate_with_schedule(WorkSchedule {
            days_per_week: dec!(4.0),
            weeks_per_year: dec!(48.0),
            months_per_year: dec!(12.0),
        });

        assert_eq!(res.hourly, dec!(20.0));
        assert_eq!(res.weekly, dec!(640.0));
        assert_eq!(res.monthly, dec!(2560.0));
        assert_eq!(res.yearly, dec!(30720.0));
    }

    #[test]
    fn calculate_decimal_rate_case() {
        let data = PayInput {
            rate: dec!(22.75),
            hours_per_day: 7,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, dec!(22.75));
        assert_eq!(res.weekly, dec!(796.25));
        assert_eq!(res.monthly.round_dp(2), dec!(3450.42));
        assert_eq!(res.yearly, dec!(41405.00));
    }

    #[test]
    fn calculate_large_input_case() {
        let data = PayInput {
            rate: dec!(9999.99),
            hours_per_day: 24,
        };

        let res = data.calculate();

        assert_eq!(res.hourly, dec!(9999.99));
        assert_eq!(res.weekly, dec!(1199998.80));
        assert_eq!(res.monthly.round_dp(2), dec!(5199994.80));
        assert_eq!(res.yearly, dec!(62399937.60));
    }
}
