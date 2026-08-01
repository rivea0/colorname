use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
pub enum Lists {
    Wikipedia,
    French,
    Spanish,
    German,
    Ridgway,
    Risograph,
    Hindi,
    Basic,
    ChineseTraditional,
    Html,
    JapaneseTraditional,
    LeCorbusier,
    NbsIscc,
    Ntc,
    Osxcrayons,
    Ral,
    SanzoWadaI,
    Thesaurus,
    Werner,
    Windows,
    X11,
    Xkcd,
    MlmcKorean,
    MlmcEnglish,
    MlmcChinese,
    MlmcRussian,
    MlmcGerman,
    MlmcSpanish,
    MlmcFinnish,
    MlmcDutch,
    MlmcPortuguese,
    MlmcRomanian,
    MlmcSwedish,
    MlmcPolish,
    MlmcPersian,
    MlmcFrench,
}

impl fmt::Display for Lists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Color in a list, as represented in data/colorlists.json.
#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    /// Color name
    pub name: String,
    /// The hex code for the color
    pub hex: String,
    /// When the user does not requests info (default behavior), the meta field should not be serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<BTreeMap<String, String>>,
}

impl Color {
    fn name(&self) -> &str {
        &self.name
    }
    fn hex(&self) -> &str {
        &self.hex
    }
    fn meta(&self) -> &Option<BTreeMap<String, String>> {
        &self.meta
    }
}

/// Data in `data/colorlists.json`
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ColorNameLists {
    pub wikipedia: Vec<Color>,
    pub french: Vec<Color>,
    pub spanish: Vec<Color>,
    pub german: Vec<Color>,
    pub ridgway: Vec<Color>,
    pub risograph: Vec<Color>,
    pub hindi: Vec<Color>,
    pub basic: Vec<Color>,
    #[serde(rename = "chineseTraditional")]
    pub chinese_traditional: Vec<Color>,
    pub html: Vec<Color>,
    #[serde(rename = "japaneseTraditional")]
    pub japanese_traditional: Vec<Color>,
    #[serde(rename = "leCorbusier")]
    pub le_corbusier: Vec<Color>,
    #[serde(rename = "nbsIscc")]
    pub nbs_iscc: Vec<Color>,
    pub ntc: Vec<Color>,
    pub osxcrayons: Vec<Color>,
    pub ral: Vec<Color>,
    #[serde(rename = "sanzoWadaI")]
    pub sanzo_wada_i: Vec<Color>,
    pub thesaurus: Vec<Color>,
    pub werner: Vec<Color>,
    pub windows: Vec<Color>,
    pub x11: Vec<Color>,
    pub xkcd: Vec<Color>,
    pub mlmc_korean: Vec<Color>,
    pub mlmc_english: Vec<Color>,
    pub mlmc_chinese: Vec<Color>,
    pub mlmc_russian: Vec<Color>,
    pub mlmc_german: Vec<Color>,
    pub mlmc_spanish: Vec<Color>,
    pub mlmc_finnish: Vec<Color>,
    pub mlmc_dutch: Vec<Color>,
    pub mlmc_portuguese: Vec<Color>,
    pub mlmc_romanian: Vec<Color>,
    pub mlmc_swedish: Vec<Color>,
    pub mlmc_polish: Vec<Color>,
    pub mlmc_persian: Vec<Color>,
    pub mlmc_french: Vec<Color>,
}

impl ColorNameLists {
    pub fn read_from_file() -> Result<Self> {
        let file = File::open("./data/colorlists.json")
            .with_context(|| "Failed to read file data/colorlists.json".to_string())?;
        let reader = BufReader::new(file);
        let lst = serde_json::from_reader(reader)?;

        Ok(lst)
    }

    pub fn get_colors_from_list(pattern: &str, list: &[Color]) -> Vec<Color> {
        let pattern = pattern.to_lowercase();
        list.iter()
            .filter(|color| color.name().to_lowercase().contains(&pattern))
            .map(|color| Color {
                name: color.name().to_string(),
                hex: color.hex().to_string(),
                meta: color.meta().clone(),
            })
            .collect::<Vec<_>>()
    }

    fn source_list_to_map() -> Result<HashMap<Lists, Vec<Color>>> {
        let data = Self::read_from_file().context("Failed parsing data")?;

        Ok(HashMap::from([
            (Lists::Wikipedia, data.wikipedia),
            (Lists::French, data.french),
            (Lists::Spanish, data.spanish),
            (Lists::German, data.german),
            (Lists::Ridgway, data.ridgway),
            (Lists::Risograph, data.risograph),
            (Lists::Hindi, data.hindi),
            (Lists::Basic, data.basic),
            (Lists::ChineseTraditional, data.chinese_traditional),
            (Lists::Html, data.html),
            (Lists::JapaneseTraditional, data.japanese_traditional),
            (Lists::LeCorbusier, data.le_corbusier),
            (Lists::NbsIscc, data.nbs_iscc),
            (Lists::Ntc, data.ntc),
            (Lists::Osxcrayons, data.osxcrayons),
            (Lists::Ral, data.ral),
            (Lists::SanzoWadaI, data.sanzo_wada_i),
            (Lists::Thesaurus, data.thesaurus),
            (Lists::Werner, data.werner),
            (Lists::Windows, data.windows),
            (Lists::X11, data.x11),
            (Lists::Xkcd, data.xkcd),
            (Lists::MlmcKorean, data.mlmc_korean),
            (Lists::MlmcEnglish, data.mlmc_english),
            (Lists::MlmcChinese, data.mlmc_chinese),
            (Lists::MlmcRussian, data.mlmc_russian),
            (Lists::MlmcGerman, data.mlmc_german),
            (Lists::MlmcSpanish, data.mlmc_spanish),
            (Lists::MlmcFinnish, data.mlmc_finnish),
            (Lists::MlmcDutch, data.mlmc_dutch),
            (Lists::MlmcPortuguese, data.mlmc_portuguese),
            (Lists::MlmcRomanian, data.mlmc_romanian),
            (Lists::MlmcSwedish, data.mlmc_swedish),
            (Lists::MlmcPolish, data.mlmc_polish),
            (Lists::MlmcPersian, data.mlmc_persian),
            (Lists::MlmcFrench, data.mlmc_french),
        ]))
    }

    pub fn search(
        pattern: &str,
        enabled_lists: Vec<Lists>,
        with_info: bool,
    ) -> Option<HashMap<Lists, Vec<Color>>> {
        let mut result = HashMap::new();

        let mapping = Self::source_list_to_map().ok()?;

        for list in enabled_lists {
            let colors = Self::get_colors_from_list(pattern, mapping.get(&list)?);
            let colors = if with_info {
                colors
            } else {
                colors
                    .into_iter()
                    .map(|color_item| Color {
                        meta: None,
                        ..color_item
                    })
                    .collect()
            };

            result.insert(list, colors);
        }

        Some(result)
    }
}
