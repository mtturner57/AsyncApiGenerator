use std::error::Error;

use clap::Parser;

pub mod enums;
pub mod services;
pub mod structs;

use enums::file_type::SupportedTypes;
use structs::options_arg::OptionsArg;
use services::path_reader::{load_path_into_string, check_file_type, check_async_version};

fn main() {    
    let args: OptionsArg = OptionsArg::parse();

    let path: &String = &args.file_path;
    let g: &String = &args.source_destination;

    dbg!(g);

    let contents: Result<String, Box<dyn Error>> = load_path_into_string(path);
    let file_type_result: Result<SupportedTypes, String> = check_file_type(path);

    match file_type_result {
        Ok(file_type) => match file_type {
            SupportedTypes:: Yaml => println!("Function coming soon!")
        },
        Err(e) => eprintln!("Error in check_file_type: {}", e)
    };

    match contents {
        Ok(value) => {
            dbg!(value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let version = check_async_version(path);
    println!("{}", &version);
}
