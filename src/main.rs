use clap::Parser;

mod command;
mod helpers;

use command::Cli;
use helpers::find_in_lists;

fn main() {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let source_list = &args.source_list;
    let with_info = args.with_info;

    let result = find_in_lists(pattern, &source_list, with_info);

    if !result.is_empty() {
        println!("{result:#?}");
    } else {
        println!("{pattern} is not found");
    }
}
