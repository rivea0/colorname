use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// Color in a list, as represented in data/colorlists.json.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Color {
    /// Color name
    pub name: String,
    /// The hex code for the color
    pub hex: String,
    /// More info about the color
    // When the user does not requests info (default behavior), the meta field should not be serialized.
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

impl fmt::Display for Lists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ColorNameLists {
    fn read_from_file() -> Result<Self> {
        let res = include_str!(concat!(env!("OUT_DIR"), "/colorlists.json"));
        let lst = serde_json::from_str(res)?;

        Ok(lst)
    }

    fn get_colors_from_list(pattern: &str, list: &[Color]) -> Vec<Color> {
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

    /// Searches for a specific pattern in given lists.
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexMap;
    /// use colorname::models::{ColorNameLists, Lists, Color};
    ///
    /// let result = ColorNameLists::search(
    ///     "pumpkin",
    ///     vec![Lists::Wikipedia, Lists::Xkcd, Lists::MlmcEnglish],
    ///     true,
    /// );
    /// let expected_result = Some(IndexMap::from([
    ///     (
    ///         Lists::Wikipedia,
    ///         vec![Color {
    ///             name: "Pumpkin".to_string(),
    ///             hex: "#ff7518".to_string(),
    ///             meta: Some(std::collections::BTreeMap::from([(
    ///                 "link".to_string(),
    ///                 "https://en.wikipedia.org/wiki/Shades_of_orange#Pumpkin".to_string()
    ///             )])),
    ///         }],
    ///     ),
    ///     (
    ///         Lists::Xkcd,
    ///         vec![
    ///             Color {
    ///                 name: "pumpkin orange".to_string(),
    ///                 hex: "#fb7d07".to_string(),
    ///                 meta: None,
    ///             },
    ///             Color {
    ///                 name: "pumpkin".to_string(),
    ///                 hex: "#e17701".to_string(),
    ///                 meta: None,
    ///             },
    ///         ],
    ///     ),
    ///     (
    ///         Lists::MlmcEnglish,
    ///         vec![
    ///             Color {
    ///                 name: "pumpkin".to_string(),
    ///                 hex: "#ee8527".to_string(),
    ///                 meta: None,
    ///             },
    ///             Color {
    ///                 name: "pumpkin orange".to_string(),
    ///                 hex: "#f38121".to_string(),
    ///                 meta: None,
    ///             },
    ///         ],
    ///     ),
    /// ]));
    ///
    /// assert_eq!(expected_result, result);
    /// ```
    pub fn search(
        pattern: &str,
        enabled_lists: Vec<Lists>,
        with_info: bool,
    ) -> Option<IndexMap<Lists, Vec<Color>>> {
        let mut result = IndexMap::new();

        let mapping = Self::source_list_to_map().ok()?;

        for list in enabled_lists {
            let colors = Self::get_colors_from_list(pattern, mapping.get(&list)?);
            if colors.is_empty() {
                continue;
            }
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

        (!result.is_empty()).then_some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_colors_from_list_returns_correct_colors() {
        let result = ColorNameLists::get_colors_from_list(
            "black",
            &[
                Color {
                    name: "pink".into(),
                    hex: "#ff69b4".into(),
                    meta: None,
                },
                Color {
                    name: "black".into(),
                    hex: "#000000".into(),
                    meta: None,
                },
            ],
        );

        assert_eq!(
            result,
            vec![Color {
                name: "black".into(),
                hex: "#000000".into(),
                meta: None
            }]
        );

        let result = ColorNameLists::get_colors_from_list(
            "black",
            &[
                Color {
                    name: "pink".into(),
                    hex: "#ff69b4".into(),
                    meta: None,
                },
                Color {
                    name: "white".into(),
                    hex: "#ffffff".into(),
                    meta: None,
                },
            ],
        );

        let empty_vec: Vec<Color> = vec![];
        assert_eq!(result, empty_vec);
    }

    #[test]
    fn search_works() -> Result<()> {
        let res =
            ColorNameLists::search("pumpkin", vec![Lists::Risograph, Lists::MlmcEnglish], false)
                .unwrap();
        {
            assert_eq!(
                res,
                IndexMap::from([
                    (
                        Lists::Risograph,
                        vec![Color {
                            name: "Pumpkin".into(),
                            hex: "#ff6f4c".into(),
                            meta: None
                        }]
                    ),
                    (
                        Lists::MlmcEnglish,
                        vec![
                            Color {
                                name: "pumpkin".into(),
                                hex: "#ee8527".into(),
                                meta: None
                            },
                            Color {
                                name: "pumpkin orange".into(),
                                hex: "#f38121".into(),
                                meta: None
                            }
                        ]
                    )
                ])
            );
        }

        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
pub enum Lists {
    Basic,
    ChineseTraditional,
    French,
    German,
    Hindi,
    Html,
    JapaneseTraditional,
    LeCorbusier,
    MlmcChinese,
    MlmcDutch,
    MlmcEnglish,
    MlmcFinnish,
    MlmcFrench,
    MlmcGerman,
    MlmcKorean,
    MlmcPersian,
    MlmcPolish,
    MlmcPortuguese,
    MlmcRomanian,
    MlmcRussian,
    MlmcSpanish,
    MlmcSwedish,
    NbsIscc,
    Ntc,
    Osxcrayons,
    Ral,
    Ridgway,
    Risograph,
    SanzoWadaI,
    Spanish,
    Thesaurus,
    Werner,
    Wikipedia,
    Windows,
    X11,
    Xkcd,
}

/// Color name lists in the collection
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ColorNameLists {
    pub basic: Vec<Color>,
    #[serde(rename = "chineseTraditional")]
    pub chinese_traditional: Vec<Color>,
    pub french: Vec<Color>,
    pub german: Vec<Color>,
    pub hindi: Vec<Color>,
    pub html: Vec<Color>,
    #[serde(rename = "japaneseTraditional")]
    pub japanese_traditional: Vec<Color>,
    #[serde(rename = "leCorbusier")]
    pub le_corbusier: Vec<Color>,
    pub mlmc_chinese: Vec<Color>,
    pub mlmc_dutch: Vec<Color>,
    pub mlmc_english: Vec<Color>,
    pub mlmc_finnish: Vec<Color>,
    pub mlmc_french: Vec<Color>,
    pub mlmc_german: Vec<Color>,
    pub mlmc_korean: Vec<Color>,
    pub mlmc_persian: Vec<Color>,
    pub mlmc_polish: Vec<Color>,
    pub mlmc_portuguese: Vec<Color>,
    pub mlmc_romanian: Vec<Color>,
    pub mlmc_russian: Vec<Color>,
    pub mlmc_spanish: Vec<Color>,
    pub mlmc_swedish: Vec<Color>,
    #[serde(rename = "nbsIscc")]
    pub nbs_iscc: Vec<Color>,
    pub ntc: Vec<Color>,
    pub osxcrayons: Vec<Color>,
    pub ral: Vec<Color>,
    pub ridgway: Vec<Color>,
    pub risograph: Vec<Color>,
    #[serde(rename = "sanzoWadaI")]
    pub sanzo_wada_i: Vec<Color>,
    pub spanish: Vec<Color>,
    pub thesaurus: Vec<Color>,
    pub werner: Vec<Color>,
    pub wikipedia: Vec<Color>,
    pub windows: Vec<Color>,
    pub x11: Vec<Color>,
    pub xkcd: Vec<Color>,
}

impl ColorNameLists {
    fn source_list_to_map() -> Result<IndexMap<Lists, Vec<Color>>> {
        let data = Self::read_from_file().context("Failed parsing data")?;
        Ok(IndexMap::from([
            (Lists::Basic, data.basic),
            (Lists::ChineseTraditional, data.chinese_traditional),
            (Lists::French, data.french),
            (Lists::German, data.german),
            (Lists::Hindi, data.hindi),
            (Lists::Html, data.html),
            (Lists::JapaneseTraditional, data.japanese_traditional),
            (Lists::LeCorbusier, data.le_corbusier),
            (Lists::MlmcChinese, data.mlmc_chinese),
            (Lists::MlmcDutch, data.mlmc_dutch),
            (Lists::MlmcEnglish, data.mlmc_english),
            (Lists::MlmcFinnish, data.mlmc_finnish),
            (Lists::MlmcFrench, data.mlmc_french),
            (Lists::MlmcGerman, data.mlmc_german),
            (Lists::MlmcKorean, data.mlmc_korean),
            (Lists::MlmcPersian, data.mlmc_persian),
            (Lists::MlmcPolish, data.mlmc_polish),
            (Lists::MlmcPortuguese, data.mlmc_portuguese),
            (Lists::MlmcRomanian, data.mlmc_romanian),
            (Lists::MlmcRussian, data.mlmc_russian),
            (Lists::MlmcSpanish, data.mlmc_spanish),
            (Lists::MlmcSwedish, data.mlmc_swedish),
            (Lists::NbsIscc, data.nbs_iscc),
            (Lists::Ntc, data.ntc),
            (Lists::Osxcrayons, data.osxcrayons),
            (Lists::Ral, data.ral),
            (Lists::Ridgway, data.ridgway),
            (Lists::Risograph, data.risograph),
            (Lists::SanzoWadaI, data.sanzo_wada_i),
            (Lists::Spanish, data.spanish),
            (Lists::Thesaurus, data.thesaurus),
            (Lists::Werner, data.werner),
            (Lists::Wikipedia, data.wikipedia),
            (Lists::Windows, data.windows),
            (Lists::X11, data.x11),
            (Lists::Xkcd, data.xkcd),
        ]))
    }
}
