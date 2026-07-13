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

#[derive(Args, Debug, Default, PartialEq, Eq)]
#[group(required = false, multiple = true)]
struct SourceList {
    #[arg(long)]
    basic: bool,
    #[arg(long, visible_alias = "zh")]
    chinese_traditional: bool,
    #[arg(long, visible_alias = "fr")]
    french: bool,
    #[arg(long, visible_alias = "de")]
    german: bool,
    #[arg(long, visible_alias = "hi")]
    hindi: bool,
    #[arg(long)]
    html: bool,
    #[arg(long, visible_alias = "ja")]
    japanese_traditional: bool,
    #[arg(long)]
    le_corbusier: bool,
    #[arg(long, visible_alias = "mlmc-zh")]
    mlmc_chinese: bool,
    #[arg(long, visible_alias = "mlmc-nl")]
    mlmc_dutch: bool,
    #[arg(long, visible_alias = "mlmc-en")]
    mlmc_english: bool,
    #[arg(long, visible_alias = "mlmc-fi")]
    mlmc_finnish: bool,
    #[arg(long, visible_alias = "mlmc-fr")]
    mlmc_french: bool,
    #[arg(long, visible_alias = "mlmc-de")]
    mlmc_german: bool,
    #[arg(long, visible_alias = "mlmc-ko")]
    mlmc_korean: bool,
    #[arg(long, visible_alias = "mlmc-fa")]
    mlmc_persian: bool,
    #[arg(long, visible_alias = "mlmc-pl")]
    mlmc_polish: bool,
    #[arg(long, visible_alias = "mlmc-pt")]
    mlmc_portuguese: bool,
    #[arg(long, visible_alias = "mlmc-ro")]
    mlmc_romanian: bool,
    #[arg(long, visible_alias = "mlmc-ru")]
    mlmc_russian: bool,
    #[arg(long, visible_alias = "mlmc-es")]
    mlmc_spanish: bool,
    #[arg(long, visible_alias = "mlmc-sv")]
    mlmc_swedish: bool,
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
    risograph: bool,
    #[arg(long)]
    sanzo_wada_i: bool,
    #[arg(long, visible_alias = "es")]
    spanish: bool,
    #[arg(long)]
    thesaurus: bool,
    #[arg(long)]
    werner: bool,
    #[arg(long)]
    windows: bool,
    #[arg(long)]
    wikipedia: bool,
    #[arg(long)]
    xkcd: bool,
    #[arg(long)]
    x11: bool,
}

fn main() {
    let args = Cli::parse();
    let s = &args.pattern;
    let source = &args.source_list;

    if *source == SourceList::default() {
        println!("Searching {s} in wikipedia..."); // default
    } else {
        println!("Searching {s} in {:?}", source);
    }
}
