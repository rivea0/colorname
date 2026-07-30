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
    #[arg(required = true)]
    pub pattern: String,
    #[command(flatten)]
    pub source_list: SourceList,
    #[arg(long)]
    pub with_info: bool,
    #[arg(value_enum, long, short = 'o')]
    pub output: Option<OutputFormats>,
    #[arg(long, short = 'q', requires = "output")]
    pub quiet: bool,
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
    #[arg(long)]
    pub all_en: bool,
    #[arg(long)]
    pub all_fr: bool,
    #[arg(long)]
    pub all_de: bool,
    #[arg(long)]
    pub all_es: bool,
    #[arg(long)]
    pub all_zh: bool,
}

impl SourceList {
    pub fn enabled_lists(&self) -> Vec<Lists> {
        let mut v = vec![];

        if self.basic {
            v.push(Lists::Basic);
        }
        if self.chinese_traditional {
            v.push(Lists::ChineseTraditional);
        }
        if self.french {
            v.push(Lists::French);
        }
        if self.german {
            v.push(Lists::German);
        }
        if self.hindi {
            v.push(Lists::Hindi);
        }
        if self.html {
            v.push(Lists::Html);
        }
        if self.japanese_traditional {
            v.push(Lists::JapaneseTraditional);
        }
        if self.le_corbusier {
            v.push(Lists::LeCorbusier);
        }
        if self.mlmc_chinese {
            v.push(Lists::MlmcChinese);
        }
        if self.mlmc_dutch {
            v.push(Lists::MlmcDutch);
        }
        if self.mlmc_english {
            v.push(Lists::MlmcEnglish);
        }
        if self.mlmc_finnish {
            v.push(Lists::MlmcFinnish);
        }
        if self.mlmc_french {
            v.push(Lists::MlmcFrench);
        }
        if self.mlmc_german {
            v.push(Lists::MlmcGerman);
        }
        if self.mlmc_korean {
            v.push(Lists::MlmcKorean);
        }
        if self.mlmc_persian {
            v.push(Lists::MlmcPersian);
        }
        if self.mlmc_polish {
            v.push(Lists::MlmcPolish);
        }
        if self.mlmc_portuguese {
            v.push(Lists::MlmcPortuguese);
        }
        if self.mlmc_romanian {
            v.push(Lists::MlmcRomanian);
        }
        if self.mlmc_russian {
            v.push(Lists::MlmcRussian);
        }
        if self.mlmc_spanish {
            v.push(Lists::MlmcSpanish);
        }
        if self.mlmc_swedish {
            v.push(Lists::MlmcSwedish);
        }
        if self.nbs_iscc {
            v.push(Lists::NbsIscc);
        }
        if self.ntc {
            v.push(Lists::Ntc);
        }
        if self.osxcrayons {
            v.push(Lists::Osxcrayons);
        }
        if self.ral {
            v.push(Lists::Ral);
        }
        if self.ridgway {
            v.push(Lists::Ridgway);
        }
        if self.risograph {
            v.push(Lists::Risograph);
        }
        if self.sanzo_wada_i {
            v.push(Lists::SanzoWadaI);
        }
        if self.spanish {
            v.push(Lists::Spanish);
        }
        if self.thesaurus {
            v.push(Lists::Thesaurus);
        }
        if self.werner {
            v.push(Lists::Werner);
        }
        if self.windows {
            v.push(Lists::Windows);
        }
        if self.wikipedia {
            v.push(Lists::Wikipedia);
        }
        if self.xkcd {
            v.push(Lists::Xkcd);
        }
        if self.x11 {
            v.push(Lists::X11);
        }
        if self.all_en {
            let en_lists = [
                Lists::Basic,
                Lists::Html,
                Lists::MlmcEnglish,
                Lists::NbsIscc,
                Lists::Ntc,
                Lists::Osxcrayons,
                Lists::Ral,
                Lists::Ridgway,
                Lists::Risograph,
                Lists::SanzoWadaI,
                Lists::Thesaurus,
                Lists::Werner,
                Lists::Windows,
                Lists::Wikipedia,
                Lists::Xkcd,
                Lists::X11,
            ];

            for list_name in en_lists {
                v.push(list_name);
            }
        }

        if self.all_fr {
            let fr_lists = [Lists::French, Lists::LeCorbusier, Lists::MlmcFrench];

            for list_name in fr_lists {
                v.push(list_name);
            }
        }

        if self.all_de {
            let de_lists = [Lists::German, Lists::MlmcGerman];

            for list_name in de_lists {
                v.push(list_name);
            }
        }

        if self.all_es {
            let es_lists = [Lists::Spanish, Lists::MlmcSpanish];

            for list_name in es_lists {
                v.push(list_name);
            }
        }

        if self.all_zh {
            let zh_lists = [Lists::ChineseTraditional, Lists::MlmcChinese];

            for list_name in zh_lists {
                v.push(list_name);
            }
        }

        // By default, list is wikipedia
        if v.is_empty() {
            v.push(Lists::Wikipedia);
        }

        v
    }
}
