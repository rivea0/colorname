use anyhow::Result;
use clap::Parser;
use colorname::models::ColorNameLists;

mod command;
mod helpers;

use crate::command::{Cli, OutputFormats};
use crate::helpers::{print_header, print_values, write_to_file};

fn main() -> Result<()> {
    let args = Cli::parse();
    let result = ColorNameLists::search(
        &args.pattern,
        args.source_list.enabled_lists(),
        args.with_info,
    );

    if let Some(result) = result {
        if let Some(output_format) = &args.output {
            match output_format {
                OutputFormats::Html => {
                    write_to_file(OutputFormats::Html, &result)?;
                }
                OutputFormats::Json => {
                    write_to_file(OutputFormats::Json, &result)?;
                }
                OutputFormats::Csv => {
                    write_to_file(OutputFormats::Csv, &result)?;
                }
            }
        }
        if !args.quiet {
            let mut with_color = true;
            if std::env::var("NO_COLOR").is_ok() {
                with_color = false;
            }

            // If truecolor is not supported, will not render as expected (https://github.com/SergioBenitez/yansi/issues/15)
            for (list_name, values) in result {
                // TODO: Use tabled output
                print_header(list_name, with_color);
                print_values(values, with_color, args.with_info)?;
            }
        }
    } else {
        println!("{} is not found", &args.pattern);
    }

    Ok(())
}
