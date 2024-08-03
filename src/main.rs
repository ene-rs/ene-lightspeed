pub mod codegen;
pub mod models;
pub mod parsers;
pub mod utils;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Lightspeed")
        .version("0.0.1")
        .author("Abdullah Sabaa Allil")
        .about("Generate a service from your DDD!")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("The input file, containing the JSON representation of the DDD"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("The output directory, where the generated service will be placed"),
        )
        .get_matches();
    let input = matches
        .get_one::<String>("input")
        .expect("You must provide an input file");
    let output = matches
        .get_one::<String>("output")
        .expect("You must provide an output directory");

    let json = std::fs::read_to_string(input).expect("Could not read the input file");
    let ddr: models::ddr_req::DomainDrivenRequest =
        serde_json::from_str(&json).expect("Could not parse the input file");
}
