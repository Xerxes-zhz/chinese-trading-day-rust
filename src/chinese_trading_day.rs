
use std::collections::HashMap;
use {
    chrono::{self, Datelike, NaiveDate, Local},
    serde::Deserialize,
    std::{env, fs::File, io::Read, path::PathBuf},
};

#[derive(Debug, Deserialize)]

struct Vocation {
    vocation: Vec<NaiveDate>,
    additional_workday: Vec<NaiveDate>,
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
struct StockCalendar{
    date_order: DateOrder,
    start_date: NaiveDate,
    end_date: NaiveDate,
    date_map: HashMap<NaiveDate, (u32,u32,u32)>,
    
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
        let start_weekday = start_date.weekday().num_days_from_monday();

        let mut prev_order: Option<DateOrder> = None;
        let mut date_order: Vec<DateOrder> = Vec::<DateOrder>::new();
        for date in start_date.iter_days().take_while(|&date| date <= end_date) {
            let if_vocation =  vocation.vocation.contains(&date);
            let if_addition_work = vocation.additional_workday.contains(&date);
            let mut if_trade = true;
            let mut if_work = true;
            let order:DateOrder = {
                if let Some(prev) = prev_order {
                    let mut working_order = prev.working_order;
                    let mut trading_order = prev.trading_order;
                    let weekday = (prev.weekday + 1u32) % 7;
                    fn one_if_even(num:u32 )-> u32{
                        if num % 2 == 0 {
                            1u32
                        } else {
                            0u32
                        }
                    }

                    if if_vocation  || weekday==5 ||weekday==6{
                        if_trade = false;
                        if_work = false;
                    } 
                    if if_addition_work{
                        if_work = true;
                    }

                    if if_trade {
                        trading_order += one_if_even(trading_order)+1u32;
                    }else{                     
                        trading_order += one_if_even(trading_order);
                    }                          
                    if if_work{               
                        working_order += one_if_even(working_order)+1u32;
                    }else{                     
                        working_order += one_if_even(working_order);
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
                    let weekday:u32  = start_weekday;
    
                    if if_vocation  || weekday==5 ||weekday==6{
                        if_trade = false;
                        if_work = false;
                    } 
                    if if_addition_work{
                        if_work = true;
                    }

                    // start with 0
                    DateOrder {
                        date,
                        calendar_order: 0u32,
                        working_order: {if if_work {0u32} else {1u32}},
                        trading_order: {if if_trade {0u32} else {1u32}},
                        weekday,
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
    fn generate_calendar() -> StockCalendar{

    }
}

pub fn naive_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
}

pub fn calendar_init(start_date:NaiveDate, end_date:NaiveDate) {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let vocation_path = PathBuf::from(&project_dir).join("data").join("vocation.yaml");

    let mut file = File::open(vocation_path).expect("Failed to open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file");

    let vocation: Vocation = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    let calendar_initializer: CalendarInitializer =
        CalendarInitializer::new(start_date, end_date, vocation);
    // for date_order in calendar_initializer.date.into_iter(){
    //     println!("{:?}", date_order);
    // }
    calendar_initializer.generate_calendar()
}
