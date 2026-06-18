# paycal

CLI pay calculator.

## Installation

First, install [Rust](https://www.rust-lang.org/tools/install).

Then clone this repo and run the CLI with your parameters:

```bash
cargo run -- <rate> <hours_per_day>
```

Arguments:

- `<rate>`: hourly pay rate
- `<hours_per_day>`: hours worked per day

You can also see the built-in help output:

```bash
cargo run -- --help
```

## Example

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

## Test

Run the test suite with:

```bash
cargo test
```
