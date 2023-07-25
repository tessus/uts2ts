# uts2ts

`uts2ts` is a simple function that does only one thing:

> It cnverts a unix timestamp to something slightly more useful. ;-)

So why then? Well, it's not always warranted to pull in a myriad of dependecies when you need this one, little thingy.

For complex time and date calculations and manipulations, please refer to the more funtional complete crates [chrono] and [time].

Please note that the `as_string()` method is just a quick way of enerating a human readable date/time string that

- is unambiguous and close to ISO 8601 (or rfc3339)
- can be used as an example how to write your own formatting function
- is NOT an attempt to reinvent all the goodies other crates provide

## Examples

```rust
use uts2ts;

fn main() {
    let ts = uts2ts::uts2ts(204158100);

    // Timestamp { year: 1976, month: 6, day: 20, hour: 22, minute: 35, second: 0, weekday: 0 }
    println!("{:?}", ts);

    // 1976-06-20 22:35:00
    println!("{}", ts.as_string());
}
```

[chrono]: https://crates.io/crates/chrono
[time]: https://crates.io/crates/time
