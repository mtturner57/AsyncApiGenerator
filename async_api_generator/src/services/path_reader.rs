use std::{error::Error, fs};
use crate::enums::file_type::SupportedTypes;

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "services/grammer.pest"]
struct FileParser;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["yaml"]; 

pub fn load_yaml_into_string(path: &String) -> Result<String, Box<dyn Error>> {
    let contents: String = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn check_file_type(path: &String) -> Result<SupportedTypes, String>{
    let file_type: Option<String> = match FileParser::parse(Rule::file_path, path){
        Ok(mut pairs) =>{
           // Get the matched pair (the entire file type)
           if let Some(pair) = pairs.next() {
                let mut inner_pairs: Pairs<'_, Rule> = pair.into_inner();
                
                // Skip filename and get the extension
                inner_pairs.next(); // This is the filename
                if let Some(extension) = inner_pairs.next() {
                    Some(extension.as_str().to_string())
                }
                else{
                    None
                }
            }
            else {
                None
            }
        }        
        Err(e) => {
            None
        }
    };

    match file_type {
        Some(ref extension) => 
            match extension.as_str() {
                "yaml" => Ok(SupportedTypes::Yaml)
            },
        None => Err(String::from("Unsupported file type or invalid string."))
    }
}

pub fn check_async_version(_path: &String) -> String {
    String::from("test")    
}