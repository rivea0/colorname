use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct Color {
    pub name: String,
    pub hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<BTreeMap<String, String>>,
}

pub trait ColorItem {
    fn name(&self) -> &str;
    fn hex(&self) -> &str;
    fn meta(&self) -> &Option<BTreeMap<String, String>>;
}

impl ColorItem for Color {
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

#[derive(Deserialize, Debug)]
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
