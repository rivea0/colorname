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

