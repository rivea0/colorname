use anyhow::Result;
use std::collections::HashMap;
use std::env::var_os;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    let lists = include_str!("./data/colorlists.json");
    // let descriptions = include_str!("./data/descriptions.json");
    let out_dir = var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("colorlists.json");

    let mut command_file = File::options().append(true).open("./src/command.rs")?;
    let mut models_file = File::options().append(true).open("./src/models.rs")?;

    let list_names = lists
        .lines()
        .map(|line| line.trim())
        .filter(|line| (!line.is_empty() || !line.starts_with("\"")) && line.contains("\": ["))
        .filter_map(|line| line.split_once("\": "))
        .filter_map(|(list_name, _)| list_name.strip_prefix("\""))
        .collect::<Vec<_>>();

    // Write `Lists`
    writeln!(
        &mut models_file,
        "#[derive(Debug, Hash, PartialEq, Eq, Serialize)]"
    )?;
    writeln!(&mut models_file, r#"pub enum Lists {{"#)?;

    for n in list_names.iter() {
        let n = title_case(n);
        writeln!(&mut models_file, "    {},", n)?;
    }
    writeln!(&mut models_file, r#"}}"#)?;
    writeln!(&mut models_file)?;

    // Write `ColorNameLists`
    writeln!(
        &mut models_file,
        r#"#[derive(Serialize, Deserialize, Default, Debug)]"#
    )?;
    writeln!(&mut models_file, r#"pub struct ColorNameLists {{"#)?;

    for n in list_names.iter() {
        if is_camel_case(n) {
            writeln!(&mut models_file, r#"    #[serde(rename = "{n}")]"#)?;
        }
        writeln!(
            &mut models_file,
            r#"    pub {}: Vec<Color>,"#,
            snake_case(n)
        )?;
    }
    writeln!(&mut models_file, r#"}}"#)?;
    writeln!(&mut models_file)?;

    // Write `ColorNameLists::source_list_to_map()`
    writeln!(&mut models_file, r#"impl ColorNameLists {{"#)?;
    writeln!(
        &mut models_file,
        r#"    fn source_list_to_map() -> Result<IndexMap<Lists, Vec<Color>>> {{"#
    )?;
    writeln!(
        &mut models_file,
        r#"        let data = Self::read_from_file().context("Failed parsing data")?;"#
    )?;
    writeln!(&mut models_file, r#"        Ok(IndexMap::from(["#)?;

    for n in list_names.iter() {
        let title_cased = title_case(n);
        let snake_cased = snake_case(n);
        writeln!(
            &mut models_file,
            r#"            (Lists::{title_cased}, data.{snake_cased}),"#
        )?;
    }

    writeln!(&mut models_file, r#"        ]))"#)?;
    writeln!(&mut models_file, r#"    }}"#)?;
    writeln!(&mut models_file, r#"}}"#)?;

    let lang_aliases = HashMap::from([
        ("german", "de"),
        ("french", "fr"),
        ("japanese", "ja"),
        ("chinese", "zh"),
        ("hindi", "hi"),
        ("spanish", "es"),
        ("english", "en"),
        ("dutch", "nl"),
        ("finnish", "fi"),
        ("korean", "ko"),
        ("persian", "fa"),
        ("polish", "pl"),
        ("portuguese", "pt"),
        ("romanian", "ro"),
        ("russian", "ru"),
        ("swedish", "sv"),
    ]);

    writeln!(
        &mut command_file,
        "#[derive(Args, Debug, Default, PartialEq, Eq)]"
    )?;
    writeln!(
        &mut command_file,
        "#[group(required = false, multiple = true)]"
    )?;
    writeln!(&mut command_file, "pub struct SourceList {{")?;

    for n in list_names.iter() {
        let n = snake_case(n);
        if let Some((_, v)) = lang_aliases.iter().find(|(k, _)| n.starts_with(*k)) {
            // TODO: add descriptions
            writeln!(
                &mut command_file,
                r#"    #[arg(long, visible_alias = "{v}")]"#
            )?;
        } else if let Some((_, v)) = lang_aliases
            .iter()
            .find(|(k, _)| n.starts_with("mlmc") && n.contains(*k))
        {
            writeln!(
                &mut command_file,
                r#"    #[arg(long, visible_alias = "mlmc-{v}")]"#
            )?;
        } else {
            writeln!(&mut command_file, r#"    #[arg(long)]"#)?;
        }
        writeln!(&mut command_file, r#"    pub {n}: bool,"#)?;
    }

    for v in ["all_en", "all_de", "all_fr", "all_es", "all_zh"] {
        writeln!(&mut command_file, r#"    #[arg(long)]"#)?;
        writeln!(&mut command_file, r#"    pub {v}: bool,"#)?;
    }

    writeln!(&mut command_file, r#"}}"#)?;
    writeln!(&mut command_file)?;

    writeln!(&mut command_file, r#"impl SourceList {{"#)?;
    writeln!(
        &mut command_file,
        r#"    pub fn enabled_lists(&self) -> Vec<Lists> {{"#
    )?;
    writeln!(&mut command_file, r#"        let mut v = vec![];"#)?;

    for n in list_names.iter() {
        let n = snake_case(n);
        writeln!(&mut command_file, r#"        if self.{n} {{"#)?;
        writeln!(
            &mut command_file,
            r#"            v.push(Lists::{});"#,
            title_case(&n)
        )?;
        writeln!(&mut command_file, r#"        }}"#)?;
    }

    writeln!(&mut command_file, r#"        if self.all_en {{"#)?;
    writeln!(&mut command_file, r#"            let en_lists = ["#)?;
    writeln!(&mut command_file, r#"                Lists::Basic,"#)?;
    writeln!(&mut command_file, r#"                Lists::Html,"#)?;
    writeln!(&mut command_file, r#"                Lists::MlmcEnglish,"#)?;
    writeln!(&mut command_file, r#"                Lists::NbsIscc,"#)?;
    writeln!(&mut command_file, r#"                Lists::Ntc,"#)?;
    writeln!(&mut command_file, r#"                Lists::Osxcrayons,"#)?;
    writeln!(&mut command_file, r#"                Lists::Ral,"#)?;
    writeln!(&mut command_file, r#"                Lists::Ridgway,"#)?;
    writeln!(&mut command_file, r#"                Lists::Risograph,"#)?;
    writeln!(&mut command_file, r#"                Lists::SanzoWadaI,"#)?;
    writeln!(&mut command_file, r#"                Lists::Thesaurus,"#)?;
    writeln!(&mut command_file, r#"                Lists::Werner,"#)?;
    writeln!(&mut command_file, r#"                Lists::Windows,"#)?;
    writeln!(&mut command_file, r#"                Lists::Wikipedia,"#)?;
    writeln!(&mut command_file, r#"                Lists::Xkcd,"#)?;
    writeln!(&mut command_file, r#"                Lists::X11,"#)?;
    writeln!(&mut command_file, r#"            ];"#)?;
    writeln!(
        &mut command_file,
        r#"            for list_name in en_lists {{"#
    )?;
    writeln!(&mut command_file, r#"                v.push(list_name);"#)?;
    writeln!(&mut command_file, r#"            }}"#)?;
    writeln!(&mut command_file, r#"        }}"#)?;
    writeln!(&mut command_file, r#"        if self.all_fr {{"#)?;
    writeln!(
        &mut command_file,
        r#"            let fr_lists = [Lists::French, Lists::LeCorbusier, Lists::MlmcFrench];"#
    )?;
    writeln!(
        &mut command_file,
        r#"            for list_name in fr_lists {{"#
    )?;
    writeln!(&mut command_file, r#"                v.push(list_name);"#)?;
    writeln!(&mut command_file, r#"            }}"#)?;
    writeln!(&mut command_file, r#"        }}"#)?;
    writeln!(&mut command_file, r#"        if self.all_de {{"#)?;
    writeln!(
        &mut command_file,
        r#"            let de_lists = [Lists::German, Lists::MlmcGerman];"#
    )?;
    writeln!(
        &mut command_file,
        r#"            for list_name in de_lists {{"#
    )?;
    writeln!(&mut command_file, r#"                v.push(list_name);"#)?;
    writeln!(&mut command_file, r#"            }}"#)?;
    writeln!(&mut command_file, r#"        }}"#)?;
    writeln!(&mut command_file, r#"        if self.all_es {{"#)?;
    writeln!(
        &mut command_file,
        r#"            let es_lists = [Lists::Spanish, Lists::MlmcSpanish];"#
    )?;
    writeln!(
        &mut command_file,
        r#"            for list_name in es_lists {{"#
    )?;
    writeln!(&mut command_file, r#"                v.push(list_name);"#)?;
    writeln!(&mut command_file, r#"            }}"#)?;
    writeln!(&mut command_file, r#"        }}"#)?;
    writeln!(&mut command_file, r#"        if self.all_zh {{"#)?;
    writeln!(
        &mut command_file,
        r#"            let zh_lists = [Lists::ChineseTraditional, Lists::MlmcChinese];"#
    )?;
    writeln!(
        &mut command_file,
        r#"            for list_name in zh_lists {{"#
    )?;
    writeln!(&mut command_file, r#"                v.push(list_name);"#)?;
    writeln!(&mut command_file, r#"            }}"#)?;
    writeln!(&mut command_file, r#"        }}"#)?;
    writeln!(
        &mut command_file,
        r#"        // By default, list is wikipedia"#
    )?;
    writeln!(&mut command_file, r#"        if v.is_empty() {{"#)?;
    writeln!(
        &mut command_file,
        r#"            v.push(Lists::Wikipedia);"#
    )?;
    writeln!(&mut command_file, r#"        }}"#)?;
    writeln!(&mut command_file, r#"        v"#)?;
    writeln!(&mut command_file, r#"    }}"#)?;
    writeln!(&mut command_file, r#"}}"#)?;

    std::fs::write(&dest_path, lists)?;
    Ok(())
}

// Snake case to title case
fn title_case(s: &str) -> String {
    s.split("_")
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut w = w.to_string();
            format!("{}{w}", w.remove(0).to_uppercase())
        })
        .collect::<Vec<_>>()
        .join("")
}

fn snake_case(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            if !res.is_empty() {
                res.push('_');
            }
            res.extend(c.to_lowercase());
        } else {
            res.push(c);
        }
    }
    res
}

fn is_camel_case(s: &str) -> bool {
    s != s.to_lowercase() && s != s.to_uppercase() && !s.contains('_')
}
