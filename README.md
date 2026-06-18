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

Optional schedule flags:

```bash
cargo run -- --rate 20 --hours 8 --days-per-week 4 --weeks-per-year 48 --months-per-year 12
```

Backwards-compatible positional form still works:

```bash
cargo run -- 20 8
cargo run -- 20 8 4 48 12
```

You can also see the built-in help output:

```bash
cargo run -- --help
```

## Examples

Default schedule:

```bash
cargo run -- --rate 20 --hours 8
```

```text
+-----------------+-----------+
| Period          | Amount    |
+-----------------+-----------+
| Hourly          |     20.00 |
| Weekly          |    800.00 |
| Monthly         |   3466.67 |
| Yearly          |  41600.00 |
+-----------------+-----------+
```

Custom schedule:

```bash
cargo run -- --rate 20 --hours 8 --days-per-week 4 --weeks-per-year 48 --months-per-year 12
```

```text
+-----------------+-----------+
| Period          | Amount    |
+-----------------+-----------+
| Hourly          |     20.00 |
| Weekly          |    640.00 |
| Monthly         |   2560.00 |
| Yearly          |  30720.00 |
+-----------------+-----------+
```

## Test

Run the test suite with:

```bash
cargo test
```
