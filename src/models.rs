use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Color {
    pub name: String,
    pub hex: String,
}

#[derive(Deserialize, Debug)]
pub struct GenericMeta {
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct ColorWithMeta<T> {
    #[serde(flatten)]
    base: Color,
    pub meta: T,
}

pub trait BaseColor {
    fn name(&self) -> &str;
    fn hex(&self) -> &str;
}

impl BaseColor for Color {
    fn name(&self) -> &str {
        &self.name
    }
    fn hex(&self) -> &str {
        &self.hex
    }
}

impl<T> BaseColor for ColorWithMeta<T> {
    fn name(&self) -> &str {
        &self.base.name
    }
    fn hex(&self) -> &str {
        &self.base.hex
    }
}

type WikipediaColor = ColorWithMeta<Option<GenericMeta>>;
type ChineseTraditionalColor = ColorWithMeta<ChineseTraditionalMeta>;
type JapaneseTraditionalColor = ColorWithMeta<JapaneseTraditionalMeta>;
type LeCorbusierColor = ColorWithMeta<LeCorbusierMeta>;
type RalColor = ColorWithMeta<RalMeta>;
type ThesaurusColor = ColorWithMeta<ThesaurusMeta>;
type WernerColor = ColorWithMeta<WernerMeta>;

#[derive(Deserialize, Debug)]
pub struct ChineseTraditionalMeta {
    pub transliteration: String,
}

#[derive(Deserialize, Debug)]
pub struct JapaneseTraditionalMeta {
    pub romanized: String,
    pub english_translation: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LeCorbusierMeta {
    pub number_suffix: String,
    pub color_number: String,
    pub collection: String,
    pub shade: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct RalMeta {
    pub rgb: String,
    pub german: String,
    pub english: String,
    pub french: String,
    pub spanish: String,
    pub italian: String,
    pub nederlands: String,
}

#[derive(Deserialize, Debug)]
pub struct ThesaurusMeta {
    pub category: String,
}

#[derive(Deserialize, Debug)]
pub struct WernerMeta {
    pub group: String,
    pub animal: String,
    pub vegetable: String,
    pub mineral: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct ColorNameLists {
    pub wikipedia: Vec<WikipediaColor>,
    pub french: Vec<ColorWithMeta<GenericMeta>>,
    pub spanish: Vec<ColorWithMeta<GenericMeta>>,
    pub german: Vec<ColorWithMeta<GenericMeta>>,
    pub ridgway: Vec<Color>,
    pub risograph: Vec<Color>,
    pub hindi: Vec<ColorWithMeta<GenericMeta>>,
    pub basic: Vec<Color>,
    #[serde(rename = "chineseTraditional")]
    pub chinese_traditional: Vec<ChineseTraditionalColor>,
    pub html: Vec<Color>,
    #[serde(rename = "japaneseTraditional")]
    pub japanese_traditional: Vec<JapaneseTraditionalColor>,
    #[serde(rename = "leCorbusier")]
    pub le_corbusier: Vec<LeCorbusierColor>,
    #[serde(rename = "nbsIscc")]
    pub nbs_iscc: Vec<Color>,
    pub ntc: Vec<Color>,
    pub osxcrayons: Vec<Color>,
    pub ral: Vec<RalColor>,
    #[serde(rename = "sanzoWadaI")]
    pub sanzo_wada_i: Vec<Color>,
    pub thesaurus: Vec<ThesaurusColor>,
    pub werner: Vec<WernerColor>,
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
