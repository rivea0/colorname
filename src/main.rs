use clap::Parser;
use yansi::Paint;
use colorname::models::ColorNameLists;

mod command;
mod helpers;

use crate::command::{Cli, OutputFormats};
use crate::helpers::{get_rgb, write_to_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let source_list = &args.source_list;
    let with_info = args.with_info;
    let output_format = &args.output;
    let is_quiet = &args.quiet;
    let result = ColorNameLists::search(pattern, source_list.enabled_lists(), with_info);

    if let Some(colors) = result {
        match output_format {
            Some(OutputFormats::Html) => {
                write_to_file(OutputFormats::Html, &colors)?;
            }
            Some(OutputFormats::Json) => {
                write_to_file(OutputFormats::Json, &colors)?;
            }
            Some(OutputFormats::Csv) => {
                write_to_file(OutputFormats::Csv, &colors)?;
            }
            None => {
                // TODO: bail
                println!("No output format given!");
            }
        }
        if !is_quiet {
            // TODO: Use tabled display
            for (list_name, values) in colors {
                println!("{}", format!("\nIn list {:?}:", list_name));
                println!("{0: <30} | {1: <7}", "name".bold(), "hex".bold(),);
                println!("----------------------------------------");
                for value in values.iter() {
                    let rgb_vals = get_rgb(&value.hex).unwrap();
                    println!(
                        "{0: <30} | {1: <7}",
                        value.name.rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2]),
                        value.hex.rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2])
                    );
                }
            }
        }
    } else {
        println!("{pattern} is not found");
    }

    Ok(())
}
