use anyhow::Result;
use clap::Parser;
use colorname::models::ColorNameLists;
use yansi::Paint;

mod command;
mod helpers;

use crate::command::{Cli, OutputFormats};
use crate::helpers::{get_rgb_value, write_to_file};

fn main() -> Result<()> {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let source_list = &args.source_list;
    let with_info = args.with_info;
    let output_format = &args.output;
    let is_quiet = &args.quiet;
    let result = ColorNameLists::search(pattern, source_list.enabled_lists(), with_info);

    if let Some(colors) = result {
        if let Some(fmt) = output_format {
            match fmt {
                OutputFormats::Html => {
                    write_to_file(OutputFormats::Html, &colors)?;
                }
                OutputFormats::Json => {
                    write_to_file(OutputFormats::Json, &colors)?;
                }
                OutputFormats::Csv => {
                    write_to_file(OutputFormats::Csv, &colors)?;
                }
            }
        }
        if !is_quiet {
            let mut with_color = true;
            if std::env::var("NO_COLOR").is_ok() {
                with_color = false;
            }

            // If truecolor is not supported, will not render as expected (https://github.com/SergioBenitez/yansi/issues/15)
            for (list_name, values) in colors {
                println!("{}", format!("\nList {:?}:", list_name));
                if !with_color {
                    println!("{0: <30} | {1: <7}", "name", "hex");
                    println!("----------------------------------------");
                } else {
                    println!(
                        "{0: <30} | {1: <7} | {2: <5}",
                        "name".bold(),
                        "hex".bold(),
                        ""
                    );
                    println!("------------------------------------------------");
                }

                for value in values.iter() {
                    if !with_color {
                        println!("{0: <30} | {1: <7}", value.name, value.hex);
                    } else {
                        let rgb_vals = get_rgb_value(&value.hex)?;
                        println!(
                            "{0: <30} | {1: <7} | {2: <5}",
                            value.name.rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2]),
                            value.hex.rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2]),
                            format!("     ").on_rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2])
                        );
                    }
                }
            }
        }
    } else {
        println!("{pattern} is not found");
    }

    Ok(())
}
