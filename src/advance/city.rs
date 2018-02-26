use std::env;
use std::fs::File;

use serde_json;
use serde::Deserialize;
use getopts;

#[derive(Debug, Deserialize)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<f64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

fn print_usage(program: &str, opts: getopts::Options) {
    println!("{}", opts.usage(&format!("Usage: {} [Options] <city>", program)))
}

pub fn demo() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Show this usage");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => panic!(err.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }

    let data_path = &matches.free[0];
    let city: &str = &matches.free[1];
    println!("program {} {} {}", program, data_path, city);

    let file = File::open(data_path).unwrap();

    let row: Row = serde_json::from_reader(file).unwrap();

    if row.city == city {
        println!("{}, {}:{:?} {:?}",
                 row.city, row.country, row.population.expect("population count"), row)
    }
}