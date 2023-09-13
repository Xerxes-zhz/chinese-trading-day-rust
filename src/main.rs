use {
    chrono::{
        self,
        Datelike,
        NaiveDate,
    },
    serde::Deserialize,
    std::{
        env,
        fs::File,
        io::Read,
        path::PathBuf,
    },
};

#[derive(Debug, Deserialize)]

struct Vocation {
    vocation:           Vec<NaiveDate>,
    additional_workday: Vec<NaiveDate>,
}

#[derive(Debug)]

struct DateSequence {
    date: Vec<(u32, NaiveDate)>,
}

struct CalendarInitializer {
    date:          Vec<(u32, NaiveDate)>,
    working_day:   Vec<(u32, NaiveDate)>,
    trading_day:   Vec<(u32, NaiveDate)>,
    start_date:    NaiveDate,
    end_date:      NaiveDate,
    start_weekday: u32,
}

impl CalendarInitializer {
    fn new(start_date: NaiveDate, end_date: NaiveDate, vocation: Vocation) -> Self {
        let all_date: Vec<(u32, NaiveDate)> = (0u32..)
            .zip(start_date.iter_days().take_while(|&date| date <= end_date))
            .collect();
        let start_weekday = start_date.weekday().num_days_from_monday();
        let working_day: Vec<(u32, NaiveDate)> = all_date
            .clone()
            .iter()
            .filter(|(index, date)| {
                !vocation.vocation.contains(date)
                    & (![5u32, 6u32].contains(&(index % 7)) | vocation.additional_workday.contains(date))
            })
            .cloned()
            .collect();
        let trading_day = working_day.clone().iter().filter(|(index , date)|{
            ![5u32, 6u32].contains(&(index%7))
        }).cloned().collect();
        Self {
            date: all_date,
            working_day,
            trading_day,
            start_date,
            end_date,
            start_weekday,
        }
    }
}

fn naive_date(date_str: &str) -> NaiveDate { NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap() }

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let vocation_path = PathBuf::from(&project_dir).join("data").join("vocation.yaml");

    let mut file = File::open(vocation_path).expect("Failed to open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file");

    let vocation: Vocation = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    let calendar_initializer: CalendarInitializer =
        CalendarInitializer::new(naive_date("2023-01-01"), naive_date("2023-06-01"), vocation);
}
