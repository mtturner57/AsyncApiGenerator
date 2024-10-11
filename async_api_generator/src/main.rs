use std::error::Error;

use clap::Parser;

pub mod enums;
pub mod services;
pub mod structs;

use enums::file_type::SupportedTypes;
use structs::options_arg::OptionsArg;
use services::path_reader::{check_async_version, check_file_type};

fn main() -> Result<(), Box<dyn Error>>{    
    let args: OptionsArg = OptionsArg::parse();
    let file_type: SupportedTypes = check_file_type(&args.file_path)?;

    let content = match file_type{
        SupportedTypes::Yaml => run_yaml_reader(args.file_path)
    }?;

    Ok(())
}

fn run_yaml_reader(path: String) -> Result<String, Box<dyn Error>>{
    let version = check_async_version(&path)?;
    println!("{}", &version);
    Ok(String::from("test"))
}
