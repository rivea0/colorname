use clap::Parser;

use colorname::models::ColorNameLists;

mod command;
mod helpers;

use crate::command::{Cli, OutputFormats};
use crate::helpers::write_to_file;

fn main() {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let source_list = &args.source_list;
    let with_info = args.with_info;
    let output_format = &args.output;
    let data = ColorNameLists::read_from_file().unwrap();
    let result = data.search(pattern, source_list.enabled_lists(), with_info);

    match output_format {
        Some(OutputFormats::Html) => {
            write_to_file(OutputFormats::Html, &result);
        }
        Some(OutputFormats::Json) => {
            write_to_file(OutputFormats::Json, &result);
        }
        Some(OutputFormats::Csv) => {
            write_to_file(OutputFormats::Csv, &result);
        }
        None => {
            // bail
            println!("No output format given!");
        }
    }

    if !result.is_empty() {
        // Use tabled display
        println!("{result:#?}");
    } else {
        // Add not found in [lists]
        println!("{pattern} is not found");
    }
}
