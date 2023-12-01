use reqwest::blocking::{Client, ClientBuilder};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use chrono::{FixedOffset, Utc, Datelike};


mod secrets;

fn get_input(year: usize, day: u32) -> String{
    static USER_AGENT: &str = "https://github.com/leoheyns";

    let url: String = format!("https://adventofcode.com/{year}/day/{day}/input");
    
    let http_client: Client = ClientBuilder::new().user_agent(USER_AGENT).build().unwrap();
    let input = http_client.get(url).header("Cookie", secrets::COOKIE)
        .send()
        .unwrap()
        .text()
        .unwrap();
    
    return input;
    }

fn gen_main(path: &str) -> String{

    let paths = fs::read_dir(path).unwrap();

    let current_days = paths
    .map(
        |p|p
        .unwrap()
        .file_name()
        .into_string()
        .unwrap()
    )
    .filter(|p|p.contains("day"))
    .map(|p| (&p[3..]).parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

    let mut result = String::new();

    result.push_str(
"use chrono::{FixedOffset, Utc, Datelike};
use std::env;
");

    result.push_str(&current_days.iter()
    .map(|day| format!(
"#[path = \"day{day}/day{day}.rs\"]
mod day{day};")
).collect::<Vec<String>>().join("\n"));



    result.push_str(
"\n\nfn run_day(day: u32){
    match day{\n");

    result.push_str(&current_days.iter().map(|day| format!("        {day}=>day{day}::run()")).collect::<Vec<String>>().join(",\n"));

    result.push_str(
",
        _=>println!(\"Soepkip! die dag bestaat niet\",)
    }
}");

    result.push_str(
"\n\nfn main() {
    let args: Vec<String> = env::args().collect();
    let timezone = FixedOffset::west_opt(5*3600).unwrap();
    let now = Utc::now();
    let datetime = now.with_timezone(&timezone);
    let mut day = datetime.date_naive().day();

    
    if args.len() == 2{
        day = args[1].parse::<u32>().unwrap()
    }
    if day < 25 && day > 0{
        run_day(day);
    }
}");

    return result;
}


fn init_day(path: &str, year:usize, day:u32){
    let _ = fs::create_dir_all(format!("{path}/day{day}"));

    let _ = OpenOptions::new().write(true).create_new(true).open(format!("{path}/day{day}/day{day}.rs")).map(|mut f| f.write_all(b"pub fn run(){\n\n}\n"));

    let _ = fs::write(format!("{path}/day{day}/input"), get_input(year, day));

    let _ = fs::write(format!("{path}/main.rs"), gen_main(path));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let timezone = FixedOffset::west_opt(5*3600).unwrap();
    let now = Utc::now();
    let datetime = now.with_timezone(&timezone);
    let mut day = datetime.date_naive().day();


    if args.len() == 2{
        day = args[1].parse::<u32>().unwrap()
    }
    if day < 25 && day > 0{
        init_day("./src", 2023, day);
    }
}
