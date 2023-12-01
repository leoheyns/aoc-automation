use chrono::{Datelike, FixedOffset, Utc};
use reqwest::blocking::{Client, ClientBuilder};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn day_str(day: u32) -> String {
    if day < 10 {
        return format!("0{day}");
    } else {
        return format!("{day}");
    }
}

fn get_input(year: usize, day: u32) -> String {
    static USER_AGENT: &str = "https://github.com/leoheyns";

    let url: String = format!("https://adventofcode.com/{year}/day/{day}/input");

    let cookie = fs::read_to_string("cookie").unwrap();

    let http_client: Client = ClientBuilder::new().user_agent(USER_AGENT).build().unwrap();
    let input = http_client
        .get(url)
        .header("Cookie", cookie)
        .send()
        .unwrap()
        .text()
        .unwrap();

    return input;
}

fn gen_main(path: &str) -> String {
    let paths = fs::read_dir(path).unwrap();

    let current_days = paths
        .map(|p| p.unwrap().file_name().into_string().unwrap())
        .filter(|p| p.contains("day"))
        .map(|p| (&p[3..]).parse::<usize>().unwrap())
        .map(|d| day_str(d as u32))
        .collect::<Vec<String>>();

    let mut result = String::new();

    result.push_str(
        "use chrono::{FixedOffset, Utc, Datelike};
use std::env;
",
    );

    result.push_str(
        &current_days
            .iter()
            .map(|dst| {
                format!(
                    "#[path = \"day{dst}/day{dst}.rs\"]
mod day{dst};"
                )
            })
            .collect::<Vec<String>>()
            .join("\n"),
    );

    result.push_str(
        "\n\nfn run_day(day: u32){
    match day{\n",
    );

    result.push_str(
        &current_days
            .iter()
            .map(|dst| format!("        {dst}=>day{dst}::run()"))
            .collect::<Vec<String>>()
            .join(",\n"),
    );

    result.push_str(
        ",
        _=>println!(\"Soepkip! die dag bestaat niet\",)
    }
}",
    );

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
}",
    );

    return result;
}

fn init_day(path: &str, year: usize, day: u32) {
    let dst = day_str(day);
    let _ = fs::create_dir_all(format!("{path}/day{dst}"));

    let _ = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{path}/day{dst}/day{dst}.rs"))
        .map(|mut f| f.write_all(b"pub fn run(){\n\n}\n"));

    let _ = fs::write(format!("{path}/day{dst}/input"), get_input(year, day));

    let _ = fs::write(format!("{path}/main.rs"), gen_main(path));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let timezone = FixedOffset::west_opt(5 * 3600).unwrap();
    let now = Utc::now();
    let datetime = now.with_timezone(&timezone);
    let mut day = datetime.date_naive().day();

    if args.len() == 2 {
        day = args[1].parse::<u32>().unwrap()
    }
    if day < 25 && day > 0 {
        init_day("./src", 2023, day);
    }
}
