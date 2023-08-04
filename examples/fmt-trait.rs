// This example shows how to add a new method to the Timestamp struct.

use uts2ts::uts2ts;

pub trait MyFormat {
    fn my_format(&self) -> String;
}

impl MyFormat for uts2ts::Timestamp {
    fn my_format(&self) -> String {
        if self.year.is_negative() {
            panic!("The method my_format() is only implemented to work for years >=0");
        }
        let weekday_name: [&str; 7] = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        format!(
            "{year:0>4}{month:0>2}{day:0>2}-{hour:0>2}{minute:0>2}{second:0>2} ({weekday})",
            year = self.year,
            month = self.month,
            day = self.day,
            hour = self.hour,
            minute = self.minute,
            second = self.second,
            weekday = weekday_name[self.weekday as usize]
        )
    }
}

fn main() {
    println!("{}", uts2ts(204158100).as_string());
    println!("{}", uts2ts(204158100).my_format());
}
