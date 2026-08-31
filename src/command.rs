use clap::{Args, Parser, ValueEnum};
use colorname::models::Lists;

#[derive(ValueEnum, PartialEq, Eq, Clone, Debug)]
pub enum OutputFormats {
    Html,
    Json,
    Csv,
}

#[derive(Parser, Debug)]
#[command(name = "colorname")]
#[command(version = "1.0")]
#[command(about = "Find colors by _name_.", long_about = None)]
pub struct Cli {
    /// Color name to search for
    #[arg(required = true)]
    pub pattern: String,
    /// List to search in
    #[command(flatten)]
    pub source_list: SourceList,
    /// Include metadata for colors if it exists
    #[arg(long)]
    pub with_info: bool,
    /// Output format to export
    #[arg(value_enum, long, short = 'o', requires = "file_path")]
    pub output: Option<OutputFormats>,
    /// Don't display output in terminal if the output format is given
    #[arg(long, short = 'q', requires = "output")]
    pub quiet: bool,
    /// File path for the output
    #[arg(long)]
    pub file_path: Option<String>,
}

