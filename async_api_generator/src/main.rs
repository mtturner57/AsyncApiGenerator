use std::error::Error;

use clap::Parser;

pub mod services;
pub mod structs;

use structs::{file_type::SupportedTypes, options_arg::OptionsArg};
use services::path_reader::{load_yaml_into_string, check_file_type};

fn main() {    
    let args: OptionsArg = OptionsArg::parse();

    let path: &String = &args.file_path;
    let g: &String = &args.source_destination;

    dbg!(g);

    let contents: Result<String, Box<dyn Error>> = load_yaml_into_string(path);
    let file_type: SupportedTypes = check_file_type(path);

    match contents {
        Ok(value) => {
            dbg!(value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }


    dbg!(&file_type);
}
