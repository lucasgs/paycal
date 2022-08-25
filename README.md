# paycal
CLI pay calculator

## Installation 

First, install [Rust](https://www.rust-lang.org/tools/install)

Then clone this repo and execute the following with your parameters:

`cargo run {rate} {hours_by_date}`

- {rate} amount pay by hour
- {hours_by_date} total hour by day

## Example

`cargo run 20 8`

```
* Results *
Hourly:  35
Weekly:  1400
Monthly: 6066
Yearly:  72800
```
## Test

Execute the following to run all the tests

`cargo test`
