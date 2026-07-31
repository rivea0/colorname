use anyhow::Result;
use clap::Parser;
use colorname::models::{Color, ColorNameLists, Lists};
use yansi::Paint;

mod command;
mod helpers;

use crate::command::{Cli, OutputFormats};
use crate::helpers::{get_rgb_value, write_to_file};

fn print_header(list_name: Lists, styled: bool) {
    println!("\nList: {list_name}");
    if styled {
        println!(
            "{0: <30} | {1: <7} | {2: <5}",
            "name".bold(),
            "hex".bold(),
            ""
        );
        println!("------------------------------------------------");
    } else {
        println!("{0: <30} | {1: <7}", "name", "hex");
        println!("------------------------------------------------");
    }
}

fn print_values(values: Vec<Color>, with_color: bool, with_info: bool) -> Result<()> {
    for value in values {
        if !with_color {
            println!("{0: <30} | {1: <7}", value.name, value.hex);
            if with_info {
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
                "     ".to_string().on_rgb(rgb_vals[0], rgb_vals[1], rgb_vals[2])
            );
            if with_info {
                for (key, val) in value.meta.as_ref().unwrap() {
                    println!("{key}: {val}");
                }
            }
        }
        println!("------------------------------------------------");
    }

    Ok(())
}

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
