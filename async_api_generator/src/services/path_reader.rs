use std::{error::Error, fs};
//, process::ExitCode
use crate::{enums::file_type::SupportedTypes, structs::version_file_contents::VersionFileContents};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "services/grammer.pest"]
struct FileParser;

pub const SUPPORTED_EXTENSIONS: [&str; 1] = ["yaml"]; 

pub fn load_path_into_string(path: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn check_file_type(path: &String) -> Result<SupportedTypes, Box<dyn Error>>{
    let path_parts: Vec<&str> = path.split(".").collect::<Vec<&str>>();

    match path_parts[path_parts.len() - 1] { // file type
        "yaml" => Ok(SupportedTypes::Yaml),
        _ => Err(Box::from(format!("Unsupported file type. Supported types are: [{}]", SUPPORTED_EXTENSIONS.join(","))))
    }
}

pub fn check_async_version(path: &str) -> Result<VersionFileContents, Box<dyn Error>> {
    let file_contents_result = load_path_into_string(path)?;

    let m : Option<String> = match FileParser::parse(Rule::async_version, &file_contents_result) {
        Ok(mut pairs) => {
            if let Some(pair) = pairs.next() {
                let mut inner_pairs: Pairs<'_, Rule> = pair.into_inner();

                if let Some(version) = inner_pairs.next() {
                    Some(version.as_str().to_string())
                }
                else{
                    None
                }
            }
            else{
                None
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    let version_contents: VersionFileContents = VersionFileContents{
        version: m.unwrap(), 
        contents: file_contents_result
    };
    
    // println!("{}", m.unwrap());
    Ok(version_contents)
}