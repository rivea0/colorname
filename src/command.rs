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

#[derive(Args, Debug, Default, PartialEq, Eq)]
#[group(required = false, multiple = true)]
pub struct SourceList {
    /// A set of basic colors, such as red, green, and blue.
    #[arg(long)]
    pub basic: bool,
    /// Traditional Colors of China: Color aesthetics in the Forbidden City (中国传统色：故宫里的色彩美学), including colors and their transliterations.
    #[arg(long, visible_alias = "zh")]
    pub chinese_traditional: bool,
    /// A list of color names in French.
    #[arg(long, visible_alias = "fr")]
    pub french: bool,
    /// A list of color names in German.
    #[arg(long, visible_alias = "de")]
    pub german: bool,
    /// A list of color names in Hindi.
    #[arg(long, visible_alias = "hi")]
    pub hindi: bool,
    /// HTML/CSS color names. These can be used as keywords in CSS, SVG, or HTML.
    #[arg(long)]
    pub html: bool,
    /// Colors traditionally used in Japanese art, literature, textiles such as kimono, and other crafts.
    #[arg(long, visible_alias = "ja")]
    pub japanese_traditional: bool,
    /// Architectural colors from Le Corbusier's color system.
    #[arg(long)]
    pub le_corbusier: bool,
    /// Chinese color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-zh")]
    pub mlmc_chinese: bool,
    /// Dutch color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-nl")]
    pub mlmc_dutch: bool,
    /// English color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-en")]
    pub mlmc_english: bool,
    /// Finnish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-fi")]
    pub mlmc_finnish: bool,
    /// French color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-fr")]
    pub mlmc_french: bool,
    /// German color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-de")]
    pub mlmc_german: bool,
    /// Korean color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-ko")]
    pub mlmc_korean: bool,
    /// Persian color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-fa")]
    pub mlmc_persian: bool,
    /// Polish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-pl")]
    pub mlmc_polish: bool,
    /// Portuguese color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-pt")]
    pub mlmc_portuguese: bool,
    /// Romanian color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-ro")]
    pub mlmc_romanian: bool,
    /// Russian color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-ru")]
    pub mlmc_russian: bool,
    /// Spanish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-es")]
    pub mlmc_spanish: bool,
    /// Swedish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum.
    #[arg(long, visible_alias = "mlmc-sv")]
    pub mlmc_swedish: bool,
    /// ISCC–NBS system of color designation based on 12 basic color terms and a small set of adjective modifiers.
    #[arg(long)]
    pub nbs_iscc: bool,
    /// NTC.js color matching library names, released in 2007. The names were collected from various sources such as Wikipedia, X11, and Crayola.
    #[arg(long)]
    pub ntc: bool,
    /// Color names used in the OS X color picker GUI.
    #[arg(long)]
    pub osxcrayons: bool,
    /// RAL color matching names.
    #[arg(long)]
    pub ral: bool,
    /// Color Nomenclature by Robert Ridgway (1850-1929), published in Washington, DC in 1912. It is part of the public domain in the USA. On August 31, 2020, it was added to the Gutenberg Project.
    #[arg(long)]
    pub ridgway: bool,
    /// Popular colors for Risograph printing, with HEX, Pantone, and Z-Type codes.
    #[arg(long)]
    pub risograph: bool,
    /// Names from Wada Sanzō (和田 三造) Colors Dictionary, Volume I.
    #[arg(long)]
    pub sanzo_wada_i: bool,
    /// A list of color names in Spanish.
    #[arg(long, visible_alias = "es")]
    pub spanish: bool,
    /// The Color Thesaurus by Ingrid Sundberg. As a writer, she collected color names to explore the emotion of a scene and create variety in her writing.
    #[arg(long)]
    pub thesaurus: bool,
    /// All colors from the book Werner's Nomenclature of Colours, collected and described in the late 18th century
    #[arg(long)]
    pub werner: bool,
    /// A list of color names scraped from Wikipedia.
    #[arg(long)]
    pub wikipedia: bool,
    /// Color names used in legacy Microsoft Windows systems.
    #[arg(long)]
    pub windows: bool,
    /// Standard Xlib or X11 protocol color names.
    #[arg(long)]
    pub x11: bool,
    /// The 954 most common RGB monitor colors, as defined by several hundred thousand participants in the XKCD color name survey.
    #[arg(long)]
    pub xkcd: bool,
    /// Search in all English language lists
    #[arg(long)]
    pub all_en: bool,
    /// Search in all German language lists
    #[arg(long)]
    pub all_de: bool,
    /// Search in all French language lists
    #[arg(long)]
    pub all_fr: bool,
    /// Search in all Spanish language lists
    #[arg(long)]
    pub all_es: bool,
    /// Search in all Chinese language lists
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
        if self.wikipedia {
            v.push(Lists::Wikipedia);
        }
        if self.windows {
            v.push(Lists::Windows);
        }
        if self.x11 {
            v.push(Lists::X11);
        }
        if self.xkcd {
            v.push(Lists::Xkcd);
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
