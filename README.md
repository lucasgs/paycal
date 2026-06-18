# paycal

CLI pay calculator.

## Installation

First, install [Rust](https://www.rust-lang.org/tools/install).

Then clone this repo and run the CLI with your parameters.

## Usage

Preferred named-flag form:

```bash
cargo run -- --rate 20 --hours 8
```

Multi-rate with shared hours:

```bash
cargo run -- --rate 20,25,30 --hours 8
```

Add an optional currency label or symbol:

```bash
cargo run -- --rate 20,25 --hours 8 --currency USD
cargo run -- --rate 20,25 --hours 8 --currency $
```

Optional schedule flags:

```bash
cargo run -- --rate 20,25 --hours 8 --days-per-week 4 --weeks-per-year 48 --months-per-year 12
```

Export as CSV or JSON:

```bash
cargo run -- --rate 20,25 --hours 8 --format csv
cargo run -- --rate 20,25 --hours 8 --format json
```

Write output directly to a file:

```bash
cargo run -- --rate 20,25 --hours 8 --format csv --output report.csv
cargo run -- --rate 20,25 --hours 8 --format json --output report.json
```

Backwards-compatible positional form still works:

```bash
cargo run -- 20 8
cargo run -- 20,25 8
cargo run -- 20,25 8 4 48 12
```

You can also see the built-in help output:

```bash
cargo run -- --help
```

## Examples

Table output:

```bash
cargo run -- --rate 20,25 --hours 8 --currency USD
```

```text
+-----------+-------------+-------------+--------------+
|      Rate |      Weekly |     Monthly |       Yearly |
+-----------+-------------+-------------+--------------+
| USD 20.00 |  USD 800.00 | USD 3466.67 | USD 41600.00 |
| USD 25.00 | USD 1000.00 | USD 4333.33 | USD 52000.00 |
+-----------+-------------+-------------+--------------+
```

CSV output:

```bash
cargo run -- --rate 20,25 --hours 8 --format csv --currency USD
```

```text
currency,USD
rate,weekly,monthly,yearly
USD 20.00,USD 800.00,USD 3466.67,USD 41600.00
USD 25.00,USD 1000.00,USD 4333.33,USD 52000.00
```

JSON output:

```bash
cargo run -- --rate 20,25 --hours 8 --format json --currency USD
```

```json
{
  "schedule": {
    "days_per_week": "5.00",
    "weeks_per_year": "52.00",
    "months_per_year": "12.00"
  },
  "currency": "USD",
  "results": [
    {
      "rate": "USD 20.00",
      "weekly": "USD 800.00",
      "monthly": "USD 3466.67",
      "yearly": "USD 41600.00"
    },
    {
      "rate": "USD 25.00",
      "weekly": "USD 1000.00",
      "monthly": "USD 4333.33",
      "yearly": "USD 52000.00"
    }
  ]
}
```

File output example:

```bash
cargo run -- --rate 20,25 --hours 8 --format csv --output report.csv
```

This writes the rendered export to `report.csv` instead of stdout.

## Test

Run the test suite with:

```bash
cargo test
```
