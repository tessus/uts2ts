# uts2ts

[![Continuous Integration](https://img.shields.io/github/actions/workflow/status/tessus/uts2ts/ci.yml?branch=master&style=flat&logo=GitHub%20Actions&logoColor=white)](https://github.com/tessus/uts2ts/actions?query=workflow%3A%22Continuous+Integration%22)
[![Continuous Deployment](https://img.shields.io/github/actions/workflow/status/tessus/uts2ts/cd.yml?style=flat&logo=GitHub%20Actions&logoColor=white&label=deploy)](https://github.com/tessus/uts2ts/actions?query=workflow%3A%22Continuous+Deployment%22)
[![GitHub Tag](https://img.shields.io/github/v/tag/tessus/uts2ts?style=flat&logo=GitHub&logoColor=white)](https://github.com/tessus/uts2ts/tags)
[![Crate Release](https://img.shields.io/crates/v/uts2ts?style=flat&logo=Rust&logoColor=white)](https://crates.io/crates/uts2ts/)
[![Documentation](https://img.shields.io/docsrs/uts2ts?style=flat&logo=Rust&logoColor=white)](https://docs.rs/uts2ts/)

`uts2ts` is a simple function that does only one thing:

> It converts a unix timestamp to something slightly more useful. ;-)

So why then? Well, it's not always warranted to pull in a myriad of dependencies when you need this one, little thingy.

For complex time and date calculations and manipulations, please refer to the more functional complete crates [chrono] and [time].

Please note that the `as_string()` method is just a quick way of generating a human readable date/time string that

- is unambiguous and close to ISO 8601 (or RFC 3339)
- can be used as an example how to write your own formatting function
- is NOT an attempt to reinvent all the goodies other crates provide

## Examples

```rust
use uts2ts::uts2ts;

fn main() {
    let ts = uts2ts(204158100);

    // Timestamp { year: 1976, month: 6, day: 20, hour: 22, minute: 35, second: 0, weekday: 0 }
    println!("{:?}", ts);

    // 1976-06-20 22:35:00
    println!("{}", ts.as_string());

    // 1976-06-20 22:35:00 +0000
    println!("{}", ts.as_string_utc());
}
```

## Maintenance Status

This library does not have any dependencies and is feature complete.

Therefore there aren't any regular updates and as a result it might seem that this project has been abandoned. However, this is not the case.

This project is maintained.

[chrono]: https://crates.io/crates/chrono
[time]: https://crates.io/crates/time
