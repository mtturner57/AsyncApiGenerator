use std::{error::Error, fs};
use crate::structs::file_type::SupportedTypes;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "services/grammer.pest"]
struct FileParser;

pub fn load_yaml_into_string(path: &String) -> Result<String, Box<dyn Error>> {
    let contents: String = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn check_file_type(path: &String) -> SupportedTypes{
    match FileParser::parse(Rule::file_path, path){
        Ok(mut pairs) =>{
           // Get the matched pair (the entire file type)
           if let Some(pair) = pairs.next() {
            let mut inner_pairs = pair.into_inner();
            
            // Skip filename and get the extension
            inner_pairs.next(); // This is the filename
            if let Some(extension) = inner_pairs.next() {
                println!("File extension: {}", extension.as_str());
            }
        } 
        }        
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    SupportedTypes::Yaml
}