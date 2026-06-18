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

Optional schedule flags:

```bash
cargo run -- --rate 20,25 --hours 8 --days-per-week 4 --weeks-per-year 48 --months-per-year 12
```

Export as CSV or JSON:

```bash
cargo run -- --rate 20,25 --hours 8 --format csv
cargo run -- --rate 20,25 --hours 8 --format json
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
cargo run -- --rate 20,25 --hours 8
```

```text
+----------+----------+----------+----------+
| Rate     | Weekly   | Monthly  | Yearly   |
+----------+----------+----------+----------+
|    20.00 |   800.00 |  3466.67 | 41600.00 |
|    25.00 |  1000.00 |  4333.33 | 52000.00 |
+----------+----------+----------+----------+
```

CSV output:

```bash
cargo run -- --rate 20,25 --hours 8 --format csv
```

```text
rate,weekly,monthly,yearly
20.00,800.00,3466.67,41600.00
25.00,1000.00,4333.33,52000.00
```

JSON output:

```bash
cargo run -- --rate 20,25 --hours 8 --format json
```

```json
{
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
