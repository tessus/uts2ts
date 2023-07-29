//! # uts2ts
//!
//! `uts2ts` is a simple function that does only one thing:
//!
//! > It converts a unix timestamp to something slightly more useful. ;-)
//!
//! So why then? Well, it's not always warranted to pull in a myriad of dependencies when you
//! need this one, little thingy.
//!
//! For complex time and date calculations and manipulations, please refer to the more functional
//! complete crates [chrono] and [time].
//!
//! Please note that the `as_string()` method is just a quick way of generating a human readable
//! date/time string that
//!
//! - is unambiguous and close to ISO 8601 (or RFC 3339)
//! - can be used as an example how to write your own formatting function
//! - is NOT an attempt to reinvent all the goodies other crates provide
//!
//! # Examples
//!
//! ```
//! use uts2ts::uts2ts;
//!
//! let ts = uts2ts(204158100);
//!
//! // Timestamp { year: 1976, month: 6, day: 20, hour: 22, minute: 35, second: 0, weekday: 0 }
//! println!("{:?}", ts);
//!
//! // 1976-06-20 22:35:00
//! println!("{}", ts.as_string());
//! ```
//! [chrono]: https://crates.io/crates/chrono
//! [time]: https://crates.io/crates/time

#[derive(Debug, Clone)]
/// The Timestamp struct that holds the date and time data.
pub struct Timestamp {
    /// values: [`i32::MIN`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN) -
    /// [`i32::MAX`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX)
    /// (in 2 billion years sombody can change this to an `i64`)
    pub year: i32,
    /// values: `1-12`
    pub month: u8,
    /// values: `1-31`
    pub day: u8,
    /// values: `0-23`
    pub hour: u8,
    /// values: `0-59`
    pub minute: u8,
    /// values: `0-59`
    pub second: u8,
    /// values: `0-6` (`0` = Sunday, `1` = Monday, ...)
    pub weekday: u8,
}

impl Timestamp {
    /// Returns a String in the format `YYYY-MM-DD hh:mm:ss`
    ///
    /// The weekday is omitted on purpose, otherwise the day strings would require localization.
    /// I chose this format, because it is unambiguous.
    ///
    /// # Examples
    ///
    /// ```
    /// use uts2ts::uts2ts;
    ///
    /// let ts = uts2ts(204158100);
    /// assert_eq!("1976-06-20 22:35:00", ts.as_string());
    ///
    /// assert_eq!("2022-12-31 23:59:59", uts2ts(1672531199).as_string());
    /// assert_eq!("2023-01-01 00:00:00", uts2ts(1672531200).as_string());
    /// assert_eq!("2024-02-29 12:34:56", uts2ts(1709210096).as_string());
    /// assert_eq!("2525-06-20 22:35:00", uts2ts(17528913300).as_string());
    /// ```
    pub fn as_string(&self) -> String {
        let mut sign = "";
        if self.year.is_negative() {
            sign = "-";
        }
        format!(
            "{sign}{year:0>4}-{month:0>2}-{day:0>2} {hour:0>2}:{minute:0>2}:{second:0>2}",
            year = self.year.abs(),
            month = self.month,
            day = self.day,
            hour = self.hour,
            minute = self.minute,
            second = self.second
        )
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn weekday(&self) -> u8 {
        self.weekday
    }
}

/// Converts a unix timestamp (also known as epoch) to a Timestamp struct.
/// The input unit is in seconds.
///
/// # Examples
///
/// ```
/// use uts2ts::uts2ts;
///
/// let ts = uts2ts(204158100);
/// // Timestamp { year: 1976, month: 6, day: 20, hour: 22, minute: 35, second: 0, weekday: 0 }
/// println!("{:?}", ts);
/// ```
pub fn uts2ts(seconds: i64) -> Timestamp {
    // Algorithm from MUSL (__secs_to_tm.c)
    const LEAPOCH: i64 = 946684800 + 86400 * (31 + 29);
    const DAYS_PER_400Y: i64 = 365 * 400 + 97;
    const DAYS_PER_100Y: i64 = 365 * 100 + 24;
    const DAYS_PER_4Y: i64 = 365 * 4 + 1;

    let secs = seconds - LEAPOCH;
    let mut days = secs / 86400;
    let mut remsecs = secs % 86400;
    if remsecs < 0 {
        remsecs += 86400;
        days -= 1;
    }

    let mut wday = (3 + days) % 7;
    if wday < 0 {
        wday += 7;
    }

    let mut qc_cycles = days / DAYS_PER_400Y;
    let mut remdays = days % DAYS_PER_400Y;
    if remdays < 0 {
        remdays += DAYS_PER_400Y;
        qc_cycles -= 1;
    }

    let mut c_cycles = remdays / DAYS_PER_100Y;
    if c_cycles == 4 {
        c_cycles -= 1;
    }
    remdays -= c_cycles * DAYS_PER_100Y;

    let mut q_cycles = remdays / DAYS_PER_4Y;
    if q_cycles == 25 {
        q_cycles -= 1;
    }
    remdays -= q_cycles * DAYS_PER_4Y;

    let mut remyears = remdays / 365;
    if remyears == 4 {
        remyears -= 1;
    }
    remdays -= remyears * 365;

    let years = remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

    let mut months: i64 = 0;
    const DAYS_IN_MONTH: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
    while DAYS_IN_MONTH[months as usize] <= remdays {
        remdays -= DAYS_IN_MONTH[months as usize];
        months += 1;
    }

    let mut ret_year = years + 2000;
    let mut ret_month = months + 2;
    if ret_month >= 12 {
        ret_month -= 12;
        ret_year += 1;
    }
    ret_month += 1;
    let ret_day = remdays + 1;
    let ret_weekday = wday;

    let ret_hour = remsecs / 3600;
    let ret_minute = remsecs / 60 % 60;
    let ret_second = remsecs % 60;

    Timestamp {
        year: ret_year as i32,
        month: ret_month as u8,
        day: ret_day as u8,
        hour: ret_hour as u8,
        minute: ret_minute as u8,
        second: ret_second as u8,
        weekday: ret_weekday as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uts2ts() {
        let ts = uts2ts(204158100);
        assert_eq!(ts.year, 1976);
        assert_eq!(ts.month, 6);
        assert_eq!(ts.day, 20);
        assert_eq!(ts.hour, 22);
        assert_eq!(ts.minute, 35);
        assert_eq!(ts.second, 0);
        assert_eq!(ts.weekday, 0);
    }

    #[test]
    fn test_uts2ts_as_string() {
        assert_eq!("1976-06-20 22:35:00", uts2ts(204158100).as_string());
        assert_eq!("2022-12-31 23:59:59", uts2ts(1672531199).as_string());
        assert_eq!("2023-01-01 00:00:00", uts2ts(1672531200).as_string());
        assert_eq!("2024-02-29 12:34:56", uts2ts(1709210096).as_string());
        assert_eq!("2525-06-20 22:35:00", uts2ts(17528913300).as_string());
        assert_eq!("-0001-12-31 23:59:59", uts2ts(-62167219201).as_string());
        assert_eq!("0000-01-01 00:00:00", uts2ts(-62167219200).as_string());
        assert_eq!("1948-03-19 15:15:15", uts2ts(-687516285).as_string());
        assert_eq!("1949-04-27 18:18:18", uts2ts(-652599702).as_string());
    }

    #[test]
    fn test_uts2ts_getters() {
        let ts = uts2ts(204158100);
        assert_eq!(ts.year(), 1976);
        assert_eq!(ts.month(), 6);
        assert_eq!(ts.day(), 20);
        assert_eq!(ts.hour(), 22);
        assert_eq!(ts.minute(), 35);
        assert_eq!(ts.second(), 0);
        assert_eq!(ts.weekday(), 0);
    }
}
