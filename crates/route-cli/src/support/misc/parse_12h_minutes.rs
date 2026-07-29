//! Helper `parse_12h_minutes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_12h_minutes(input: &str) -> Option<i32> {
    let input = input.trim();
    let (time, suffix) = input.rsplit_once(' ')?;
    let (hour, minute) = time.split_once(':')?;
    let mut hour = hour.parse::<i32>().ok()?;
    let minute = minute.parse::<i32>().ok()?;
    if !(1..=12).contains(&hour) || !(0..=59).contains(&minute) {
        return None;
    }
    let suffix = suffix.to_ascii_uppercase();
    if suffix == "PM" && hour != 12 {
        hour += 12;
    } else if suffix == "AM" && hour == 12 {
        hour = 0;
    } else if suffix != "AM" && suffix != "PM" {
        return None;
    }
    Some(hour * 60 + minute)
}

