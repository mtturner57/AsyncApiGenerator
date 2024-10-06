use clap::Parser;

#[derive(Parser,Debug)]
pub struct OptionsArg{
    #[clap(short = 'p', long = "file-path")]
    pub file_path: String,

    #[clap(short = 's', long = "source-destination")]
    pub source_destination: String
}