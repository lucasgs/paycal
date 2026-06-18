# paycal

CLI pay calculator.

## Installation

First, install [Rust](https://www.rust-lang.org/tools/install).

Then clone this repo and run the CLI with your parameters:

```bash
cargo run -- <rate> <hours_per_day> [days_per_week] [weeks_per_year] [months_per_year]
```

Arguments:

- `<rate>`: hourly pay rate
- `<hours_per_day>`: hours worked per day
- `[days_per_week]`: optional work days per week, default `5`
- `[weeks_per_year]`: optional work weeks per year, default `52`
- `[months_per_year]`: optional months per year, default `12`

You can also see the built-in help output:

```bash
cargo run -- --help
```

## Examples

Default schedule:

```bash
cargo run -- 20 8
```

```text
* Results *
Hourly:  20.00
Weekly:  800.00
Monthly: 3466.67
Yearly:  41600.00
```

Custom schedule (4 days/week, 48 weeks/year, 12 months/year):

```bash
cargo run -- 20 8 4 48 12
```

```text
* Results *
Hourly:  20.00
Weekly:  640.00
Monthly: 2560.00
Yearly:  30720.00
```

## Test

Run the test suite with:

```bash
cargo test
```
