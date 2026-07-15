use clap::{Args, Parser, ValueEnum};

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
    pub pattern: String,
    #[command(flatten)]
    pub source_list: SourceList,
    #[arg(long)]
    pub with_info: bool,
    #[arg(value_enum, long, short = 'o')]
    pub output: Option<OutputFormats>
}

#[derive(Args, Debug, Default, PartialEq, Eq)]
#[group(required = false, multiple = true)]
pub struct SourceList {
    #[arg(long)]
    pub basic: bool,
    #[arg(long, visible_alias = "zh")]
    pub chinese_traditional: bool,
    #[arg(long, visible_alias = "fr")]
    pub french: bool,
    #[arg(long, visible_alias = "de")]
    pub german: bool,
    #[arg(long, visible_alias = "hi")]
    pub hindi: bool,
    #[arg(long)]
    pub html: bool,
    #[arg(long, visible_alias = "ja")]
    pub japanese_traditional: bool,
    #[arg(long)]
    pub le_corbusier: bool,
    #[arg(long, visible_alias = "mlmc-zh")]
    pub mlmc_chinese: bool,
    #[arg(long, visible_alias = "mlmc-nl")]
    pub mlmc_dutch: bool,
    #[arg(long, visible_alias = "mlmc-en")]
    pub mlmc_english: bool,
    #[arg(long, visible_alias = "mlmc-fi")]
    pub mlmc_finnish: bool,
    #[arg(long, visible_alias = "mlmc-fr")]
    pub mlmc_french: bool,
    #[arg(long, visible_alias = "mlmc-de")]
    pub mlmc_german: bool,
    #[arg(long, visible_alias = "mlmc-ko")]
    pub mlmc_korean: bool,
    #[arg(long, visible_alias = "mlmc-fa")]
    pub mlmc_persian: bool,
    #[arg(long, visible_alias = "mlmc-pl")]
    pub mlmc_polish: bool,
    #[arg(long, visible_alias = "mlmc-pt")]
    pub mlmc_portuguese: bool,
    #[arg(long, visible_alias = "mlmc-ro")]
    pub mlmc_romanian: bool,
    #[arg(long, visible_alias = "mlmc-ru")]
    pub mlmc_russian: bool,
    #[arg(long, visible_alias = "mlmc-es")]
    pub mlmc_spanish: bool,
    #[arg(long, visible_alias = "mlmc-sv")]
    pub mlmc_swedish: bool,
    #[arg(long)]
    pub nbs_iscc: bool,
    #[arg(long)]
    pub ntc: bool,
    #[arg(long)]
    pub osxcrayons: bool,
    #[arg(long)]
    pub ral: bool,
    #[arg(long)]
    pub ridgway: bool,
    #[arg(long)]
    pub risograph: bool,
    #[arg(long)]
    pub sanzo_wada_i: bool,
    #[arg(long, visible_alias = "es")]
    pub spanish: bool,
    #[arg(long)]
    pub thesaurus: bool,
    #[arg(long)]
    pub werner: bool,
    #[arg(long)]
    pub windows: bool,
    #[arg(long)]
    pub wikipedia: bool,
    #[arg(long)]
    pub xkcd: bool,
    #[arg(long)]
    pub x11: bool,
}
