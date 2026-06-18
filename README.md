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

Sort comparison rows:

```bash
cargo run -- --rate 25,20,30 --hours 8 --sort yearly
cargo run -- --rate 25,20,30 --hours 8 --sort rate
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

Sorted table output:

```bash
cargo run -- --rate 25,20,30 --hours 8 --sort yearly
```

```text
+-------+---------+---------+----------+
|  Rate |  Weekly | Monthly |   Yearly |
+-------+---------+---------+----------+
| 20.00 |  800.00 | 3466.67 | 41600.00 |
| 25.00 | 1000.00 | 4333.33 | 52000.00 |
| 30.00 | 1200.00 | 5200.00 | 62400.00 |
+-------+---------+---------+----------+
```

CSV output with metadata:

```bash
cargo run -- --rate 25,20 --hours 8 --format csv --sort rate
```

```text
format,csv
hours_per_day,8
days_per_week,5.00
weeks_per_year,52.00
months_per_year,12.00
generated_at_unix_seconds,1712345678
sort,rate
rate,weekly,monthly,yearly
20.00,800.00,3466.67,41600.00
25.00,1000.00,4333.33,52000.00
```

JSON output with metadata:

```bash
cargo run -- --rate 25,20 --hours 8 --format json --sort yearly
```

```json
{
  "metadata": {
    "format": "json",
    "hours_per_day": 8,
    "generated_at_unix_seconds": 1712345678,
    "sort": "yearly"
  },
  "schedule": {
    "days_per_week": "5.00",
    "weeks_per_year": "52.00",
    "months_per_year": "12.00"
  },
  "results": [
    {
      "rate": "20.00",
      "weekly": "800.00",
      "monthly": "3466.67",
      "yearly": "41600.00"
    },
    {
      "rate": "25.00",
      "weekly": "1000.00",
      "monthly": "4333.33",
      "yearly": "52000.00"
    }
  ]
}
```

## Test

Run the test suite with:

```bash
cargo test
```
