use chrono::prelude::*;
use chrono::{ Date, Utc};
pub fn calculate_age(person_date: Date<Utc>)-> String{
  let current = Utc::now().date();
  let dur = current.signed_duration_since(person_date);
  if dur.num_days() < 0 {
      panic!("birth date exeeding current date")
  }
  let mut current_date =  current.day() as i32;
  let mut current_month = current.month()as i32;
  let mut current_year  = current.year()as i32;
  let  birth_date = person_date.day()as i32;
  let  birth_month = person_date.month()as i32;
  let  birth_year = person_date.year()as i32;
  let month:Vec<i32>= vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; 
  if birth_date > current_date{
      current_date = current_date + month[(birth_month-1) as usize];
      current_month = current_month - 1; 
  }
  if birth_month > current_month { 
      current_year = current_year - 1; 
      current_month = current_month + 12; 
  } 
  let  calculated_date = current_date - birth_date; 
  let calculated_month = current_month - birth_month; 
  let  calculated_year = current_year - birth_year; 
  format!("{}years {}month {}days",calculated_year, calculated_month, calculated_date )
}



#[test]
fn test1_calculate_age() {
  let  date = NaiveDate::parse_from_str("2020-01-15", "%Y-%m-%d");
  let  test_date = Utc.from_local_date(&date.unwrap()).unwrap();
    assert_eq!(calculate_age(test_date), "0years 5month 1days");
}

#[test]
#[should_panic(expected = "birth date exeeding current date")]
fn test2_calculate_age() {
  let  date = NaiveDate::parse_from_str("2021-01-15", "%Y-%m-%d");
  let  test_date = Utc.from_local_date(&date.unwrap()).unwrap();
    calculate_age(test_date);
}
