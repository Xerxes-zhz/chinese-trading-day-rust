use {
    chrono::{self, Datelike, NaiveDate},
    serde::Deserialize,
    std::{env, fs::File, io::Read, path::PathBuf},
};

#[derive(Debug, Deserialize)]

struct Vocation {
    vocation: Vec<NaiveDate>,
    additional_workday: Vec<NaiveDate>,
}

enum OddEven {
    Odd,
    Even
}
#[derive(Debug, Clone)]

struct DateOrder {
    date: NaiveDate,
    calendar_order: u32,
    working_order: u32,
    trading_order: u32,
    weekday: u32,
}
#[derive(Debug)]

struct CalendarInitializer {
    date: Vec<DateOrder>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_weekday: u32,
}

impl CalendarInitializer {
    fn new(start_date: NaiveDate, end_date: NaiveDate, vocation: Vocation) -> Self {
        let all_date: Vec<(u32, NaiveDate)> = (0u32..)
            .zip(start_date.iter_days().take_while(|&date| date <= end_date))
            .collect();
        let start_weekday = start_date.weekday().num_days_from_monday();

        let mut prev_order: Option<DateOrder> = None;
        let mut date_order: Vec<DateOrder> = Vec::<DateOrder>::new();
        for date in start_date.iter_days().take_while(|&date| date <= end_date) {
            let order:DateOrder = {
                if let Some(prev) = prev_order {
                    let mut working_order = prev.working_order;
                    let mut trading_order = prev.trading_order;
                    let weekday = (prev.weekday + 1u32) % 7;
                    fn one_if(num:u32,odd_or_even:OddEven) -> u32{
                        if num % 2 == {
                            match odd_or_even {
                            OddEven::Even=>{0}
                            OddEven::Odd=>{1}
                        } }{
                            1u32
                        } else {
                            0u32
                        }
                    }
                    if vocation.vocation.contains(&date) {
                        working_order += one_if(working_order,OddEven::Even) ;
                        trading_order += one_if(trading_order,OddEven::Even) ;
                    } else {
                        if weekday==5 || weekday==6{
                        trading_order += one_if(trading_order,OddEven::Even) ;
                            if vocation.additional_workday.contains(&date){
                                working_order += one_if(working_order,OddEven::Odd)+1u32;
                            }else{
                                working_order+=one_if(working_order,OddEven::Even);
                            }
                        }else{
                            working_order += one_if(working_order,OddEven::Odd)+1u32;
                            trading_order += one_if(working_order,OddEven::Odd)+1u32;
                        }
                    }

                    DateOrder {
                        date,
                        // date add 2 each day
                        calendar_order: prev.calendar_order + 2u32,
                        working_order,
                        trading_order,
                        weekday,
                    }
                } else {
                    // start with 0
                    DateOrder {
                        date,
                        calendar_order: 0u32,
                        working_order: 0u32,
                        trading_order: 0u32,
                        weekday: start_weekday,
                    }
                }
            };
            date_order.push(order.clone());
            prev_order = Some(order);

        }

        Self {
            date: date_order,
            start_date,
            end_date,
            start_weekday,
        }
    }
}

fn naive_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
}

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let vocation_path = PathBuf::from(&project_dir).join("data").join("vocation.yaml");

    let mut file = File::open(vocation_path).expect("Failed to open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file");

    let vocation: Vocation = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    let calendar_initializer: CalendarInitializer =
        CalendarInitializer::new(naive_date("2023-01-01"), naive_date("2023-06-01"), vocation);
    for date_order in calendar_initializer.date.into_iter(){
        println!("{:?}", date_order);
    }
}
