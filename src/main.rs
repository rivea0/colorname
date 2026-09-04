use anyhow::Result;
use clap::Parser;
use colorname::models::ColorNameLists;
use comfy_table::{Table, presets::UTF8_FULL};

mod command;
mod helpers;

use crate::command::Cli;
use crate::helpers::{set_table_header, set_table_rows, write_to_file};

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

            // If truecolor is not supported, will not render as expected (https://github.com/SergioBenitez/yansi/issues/15)
            for (list_name, values) in result {
                let mut table = Table::new();
                table.load_style(UTF8_FULL.with_solid_inner_borders());
                set_table_header(&mut table, list_name, with_color);
                set_table_rows(&mut table, values, with_color, args.with_info)?;

                println!("{table}");
            }
        }
    } else {
        println!("{} is not found", &args.pattern);
    }

    Ok(())
}
