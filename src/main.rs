use chrono::{self, NaiveDate};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
#[derive(Debug, Deserialize)]
struct Vocation {
    vocation: Vec<String>,
    additional_workday: Vec<String>,
}
fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let vocation_path = PathBuf::from(&project_dir)
        .join("data")
        .join("vocation.yaml");
    let mut  file = File::open(vocation_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let vocation: Vocation = serde_yaml::from_str(&contents).expect("Failed to parse YAML");   
    println!("{:?}",contents); 
}
