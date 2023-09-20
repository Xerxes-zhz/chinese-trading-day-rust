use chinese_trading_day::{calendar_init, naive_date};
use chrono::{self, Datelike, NaiveDate, Local};

fn main(){
    println!("now: {}",Local::now());
    calendar_init(naive_date("2000-01-01"), naive_date("2070-06-01"));
    println!("now: {}",Local::now());
}
