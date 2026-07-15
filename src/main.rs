use clap::Parser;

mod command;
mod helpers;

use command::{Cli, OutputFormats};
use helpers::{find_in_lists, write_to_file};

fn main() {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let source_list = &args.source_list;
    let with_info = args.with_info;
    let output_format = &args.output;

    let result = find_in_lists(pattern, &source_list, with_info);
    match output_format {
        Some(OutputFormats::Html) => {
            write_to_file(OutputFormats::Html);
        },
        Some(OutputFormats::Json) => {
            write_to_file(OutputFormats::Json);
        },
        Some(OutputFormats::Csv) => {
            write_to_file(OutputFormats::Csv);
        },
        None => {
            println!("No output format given!");
        }
    }

    if !result.is_empty() {
        println!("{result:#?}");
    } else {
        println!("{pattern} is not found");
    }
}
