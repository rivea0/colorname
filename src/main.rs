use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(name = "colorname")]
#[command(version = "1.0")]
#[command(about = "Find colors by _name_.", long_about = None)]
struct Cli {
    pattern: String,
    #[command(flatten)]
    source_list: SourceList,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
struct SourceList {
    #[arg(long)]
    basic: bool,
    #[arg(long)]
    html: bool,
    #[arg(long, visible_alias = "ja")]
    japanese_traditional: bool,
    #[arg(long)]
    le_corbusier: bool,
    #[arg(long)]
    nbs_iscc: bool,
    #[arg(long)]
    ntc: bool,
    #[arg(long)]
    osxcrayons: bool,
    #[arg(long)]
    ral: bool,
    #[arg(long)]
    ridgway: bool,
    #[arg(long)]
    sanzo_wada_1: bool,
    #[arg(long)]
    thesaurus: bool,
    #[arg(long)]
    werner: bool,
    #[arg(long)]
    windows: bool,
    #[arg(long)]
    wikipedia: bool,
    #[arg(long, visible_alias = "fr")]
    french: bool,
    #[arg(long, visible_alias = "es")]
    spanish: bool,
    #[arg(long, visible_alias = "de")]
    german: bool,
    #[arg(long)]
    x11: bool,
    #[arg(long)]
    xkcd: bool,
    #[arg(long)]
    risograph: bool,
    #[arg(long, visible_alias = "zh")]
    chinese_traditional: bool,
    #[arg(long, visible_alias = "hi")]
    hindi: bool,
}

fn main() {
    let args = Cli::parse();
    let s = &args.pattern;
    let source = &args.source_list;
    println!("Searching for {s} in {source:?}");
}
