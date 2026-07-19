use clap::Parser;

use colorname::models::ColorNameLists;

mod command;
mod helpers;

use crate::command::{Cli, OutputFormats};
use crate::helpers::write_to_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let source_list = &args.source_list;
    let with_info = args.with_info;
    let output_format = &args.output;
    let result = ColorNameLists::search(pattern, source_list.enabled_lists(), with_info);

    if let Some(colors) = result {
        // TODO: Use tabled display
        println!("{colors:#?}");
        match output_format {
            Some(OutputFormats::Html) => {
                write_to_file(OutputFormats::Html, &colors);
            }
            Some(OutputFormats::Json) => {
                write_to_file(OutputFormats::Json, &colors);
            }
            Some(OutputFormats::Csv) => {
                write_to_file(OutputFormats::Csv, &colors);
            }
            None => {
                // TODO: bail
                println!("No output format given!");
            }
        }
    } else {
        println!("{pattern} is not found");
    }

    Ok(())
}
