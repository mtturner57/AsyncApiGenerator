use std::{error::Error, ptr::null};

use clap::Parser;

pub mod enums;
pub mod services;
pub mod structs;

use enums::file_type::SupportedTypes;
use structs::{options_arg::OptionsArg, version_file_contents::VersionFileContents};
use services::{path_reader::{check_async_version, check_file_type}, async_readers::{yaml_3_reader::createV3}};

fn main() -> Result<(), Box<dyn Error>>{    
    let args: OptionsArg = OptionsArg::parse();
    let file_type: SupportedTypes = check_file_type(&args.file_path)?;

    let _content = match file_type{
        SupportedTypes::Yaml => run_yaml_reader(&args.file_path)
    }?;

    Ok(())
}

fn run_yaml_reader(path: &str) -> Result<String, Box<dyn Error>>{
    let version_contents: VersionFileContents = check_async_version(path)?;
    
    let mat = match version_contents.version.as_str() {
        "3.0.0" => createV3(&version_contents),
        _ => Err(Box::from(format!("Error retrieving the version. No matching version supported.")))
    };

    mat
}