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
                        if args.with_info {
                            for (key, val) in value.meta.as_ref().unwrap() {
                                println!("{key}: {val}\n");
                            }
                        }
                    } else {
                        let rgb_vals = get_rgb_value(&value.hex)?;
                        println!(
                            "{0: <30} | {1: <7} | {2: <5}",
                            value.name.rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2]),
                            value.hex.rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2]),
                            format!("     ").on_rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2])
                        );
                        if args.with_info {
                            for (key, val) in value.meta.as_ref().unwrap() {
                                println!("{key}: {val}\n");
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("{} is not found", &args.pattern);
    }

    Ok(())
}
