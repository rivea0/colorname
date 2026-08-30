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

