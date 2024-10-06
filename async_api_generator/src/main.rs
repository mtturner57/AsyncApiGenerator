use clap::Parser;
use structs::options_arg::OptionsArg;

pub mod structs;

fn main() {    
    let args: OptionsArg = OptionsArg::parse();

    let g: &String = &args.source_destination;

    dbg!(g);
}
