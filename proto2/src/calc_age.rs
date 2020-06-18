use chrono::prelude::*;
use std::time::SystemTime;
pub fn calculate_age(person_date: NaiveDate, present_date: NaiveDate) -> String {
    let dur = present_date.signed_duration_since(person_date);
    if dur.num_days() < 0 {
        panic!("birth date exceeding current date")
    }
    let mut current_date = present_date.day() as i32;
    let mut current_month = present_date.month() as i32;
    let mut current_year = present_date.year() as i32;
    let birth_date = person_date.day() as i32;
    let birth_month = person_date.month() as i32;
    let birth_year = person_date.year() as i32;
    let months_def: Vec<i32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if birth_date > current_date {
        current_date += months_def[(birth_month - 1) as usize];
        current_month -= 1;
    }
    if birth_month > current_month {
        current_year -= 1;
        current_month += 12;
    }
    let calculated_date = current_date - birth_date;
    let calculated_month = current_month - birth_month;
    let calculated_year = current_year - birth_year;
    format!(
        "{}years {}month {}days",
        calculated_year, calculated_month, calculated_date
    )
}

macro_rules! calculate_age {
    ($person_date: expr,$current: expr) => {
        $crate::calc_age::calculate_age($person_date, $current);
    };
    ($person_date: expr) => {
        $crate::calc_age::calculate_age($person_date, $crate::calc_age::get_current_date());
    };
}
pub fn get_current_date() -> NaiveDate {
    let current_system_time: SystemTime = SystemTime::now();
    let current_date_time: DateTime<Utc> = DateTime::from(current_system_time);
    current_date_time.naive_local().date()
}

#[test]
fn test1_calculate_age() {
    let from_date =
        NaiveDate::parse_from_str("1999-01-09", "%Y-%m-%d").expect("unable to parse from date");
    let current_date =
        NaiveDate::parse_from_str("2020-06-18", "%Y-%m-%d").expect("unable to parse current date");
    assert_eq!(
        calculate_age!(from_date, current_date),
        "21years 5month 9days"
    );
}

#[test]
#[should_panic(expected = "birth date exceeding current date")]
fn test2_calculate_age() {
    let from_date =
        NaiveDate::parse_from_str("2021-01-15", "%Y-%m-%d").expect("unable to parse from date");
    let current_date =
        NaiveDate::parse_from_str("2020-01-15", "%Y-%m-%d").expect("unable to parse current date");

    calculate_age!(from_date, current_date);
}
#[test]
fn test3_calculate_age() {
    let from_date =
        NaiveDate::parse_from_str("1999-01-09", "%Y-%m-%d").expect("unable to parse from date");
    let current_date =
        NaiveDate::parse_from_str("1999-01-10", "%Y-%m-%d").expect("unable to parse current date");

    assert_eq!(
        calculate_age!(from_date, current_date),
        "0years 0month 1days"
    );
}
#[test]
#[should_panic(expected = "unable to parse from date")]
fn test4_calculate_age() {
    let from_date =
        NaiveDate::parse_from_str("2021-01", "%Y-%m-%d").expect("unable to parse from date");
    let current_date =
        NaiveDate::parse_from_str("2020-01-15", "%Y-%m-%d").expect("unable to parse current date");

    calculate_age!(from_date, current_date);
}
