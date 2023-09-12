use chrono::{self, Datelike, Duration, NaiveDate};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Vocation {
    vocation: Vec<NaiveDate>,
    additional_workday: Vec<NaiveDate>,
}
#[derive(Debug)]
struct DateSequence {
    date: Vec<(u32, NaiveDate)>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_weekday: u32,
}
trait Convertible {
    fn from_sequence(ds: DateSequence) -> Self;

    fn to_sequence(self) -> DateSequence;
}
struct Workday(DateSequence);
struct DateRange(DateSequence);
impl Convertible for DateRange {
    fn from_sequence(ds: DateSequence) -> Self {
        DateRange(ds)
    }
    fn to_sequence(self) -> DateSequence {
        self.0
    }
}
impl Convertible for Workday {
    fn from_sequence(ds: DateSequence) -> Self {
        Workday(ds)
    }
    fn to_sequence(self) -> DateSequence {
        self.0
    }
}
impl DateSequence {
    fn convert<T: Convertible, U: Convertible>(t: T) -> U {
        U::from_sequence(t.to_sequence())
    }
}
impl DateRange {
    fn new(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let date = (0u32..)
            .zip(start_date.iter_days().take_while(|&date| date <= end_date))
            .collect();
        Self::from_sequence(DateSequence {
            start_date,
            end_date,
            date,
            start_weekday: start_date.weekday().num_days_from_monday(),
        })
    }
}
impl Workday {
    fn new(date_range: DateRange, vocation: Vocation) -> Self {
        let mut  workday: Workday = DateSequence::convert(date_range);
        workday.0.date = workday
            .0
            .date
            .iter()
            .filter(|(index, date)| {
                !vocation.vocation.contains(date)
                    & (![5u32, 6u32].contains(&(index % 7))
                        | vocation.additional_workday.contains(date))
            })
            .cloned()
            .collect();
        workday
        
    }
}

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let vocation_path = PathBuf::from(&project_dir)
        .join("data")
        .join("vocation.yaml");
    let mut file = File::open(vocation_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let vocation: Vocation = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    let start_date = NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str("2023-06-01", "%Y-%m-%d").unwrap();
    let all_date = DateRange::new(start_date, end_date);
    let workday = Workday::new(all_date, vocation);

    println!("workday {:?}", workday.0);
}
