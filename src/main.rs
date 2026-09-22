use anyhow::Result;
use clap::Parser;
use colorname::models::ColorNameLists;

mod command;
mod helpers;

use crate::command::Cli;
use crate::helpers::{create_table, write_to_file};

fn main() -> Result<()> {
    let args = Cli::parse();
    let result = ColorNameLists::search(
        &args.pattern,
        args.source_list.enabled_lists(),
        args.with_info,
    );

    if let Some(result) = result {
        if let (Some(output_format), Some(file_path)) = (args.output, args.file_path) {
            write_to_file(output_format, &result, file_path)?;
        }
        if !args.quiet {
            let mut with_color = true;
            if std::env::var("NO_COLOR").is_ok() {
                with_color = false;
            }

            for (list_name, values) in result {
                println!("\nList: {list_name}");
                let table = create_table(values, with_color, args.with_info)?;
                println!("{table}");
            }
        }
    } else {
        println!("{} is not found", args.pattern);
    }

    Ok(())
}
