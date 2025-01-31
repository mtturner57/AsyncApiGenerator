use std::error::Error;

use crate::structs::version_file_contents::VersionFileContents;



pub fn createV3(contents: &VersionFileContents) -> Result<String, Box<dyn Error>>{
    Ok(String::from("test"))
}