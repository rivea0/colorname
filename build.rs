use anyhow::Result;
use std::env::var_os;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    let s = include_str!("./data/colorlists.json");
    let out_dir = var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("colorlists.json");

    // let mut command_file = File::options().append(true).open("./src/command.rs")?;
    let mut models_file = File::options().append(true).open("./src/models.rs")?;

    // Write `Lists`
    writeln!(
        &mut models_file,
        "#[derive(Debug, Hash, PartialEq, Eq, Serialize)]"
    )?;
    writeln!(&mut models_file, r#"pub enum Lists {{"#)?;

    for (_idx, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with("\"") {
            continue;
        }
        if line.contains("\": [") {
            let (list_name, _) = line
                .split_once("\": ")
                .ok_or_else(|| anyhow::anyhow!("Failed to parse list name"))?;
            if let Some(list_name) = list_name.strip_prefix("\"") {
                let list_name = title_case_list_name(&list_name);
                writeln!(&mut models_file, "    {},", list_name)?;
            }
        }
    }
    writeln!(&mut models_file, r#"}}"#)?;
    writeln!(&mut models_file)?;

    // Write `ColorNameLists`
    writeln!(
        &mut models_file,
        r#"#[derive(Serialize, Deserialize, Default, Debug)]"#
    )?;
    write!(&mut models_file, r#"pub struct ColorNameLists {{"#)?;
    for (_idx, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with("\"") {
            continue;
        }
        if line.contains("\": [") {
            let (list_name, _) = line
                .split_once("\": ")
                .ok_or_else(|| anyhow::anyhow!("Failed to parse list name"))?;
            if let Some(list_name) = list_name.strip_prefix("\"") {
                match list_name {
                    "chineseTraditional" => {
                        writeln!(
                            &mut models_file,
                            r#"    #[serde(rename = "chineseTraditional")]"#
                        )?;
                    }
                    "japaneseTraditional" => {
                        writeln!(
                            &mut models_file,
                            r#"    #[serde(rename = "japaneseTraditional")]"#
                        )?;
                    }
                    "leCorbusier" => {
                        writeln!(&mut models_file, r#"    #[serde(rename = "leCorbusier")]"#)?;
                    }
                    "nbsIscc" => {
                        writeln!(&mut models_file, r#"    #[serde(rename = "nbsIscc")]"#)?;
                    }
                    "sanzoWadaI" => {
                        writeln!(&mut models_file, r#"    #[serde(rename = "sanzoWadaI")]"#)?;
                    }
                    _ => {}
                }
                writeln!(
                    &mut models_file,
                    r#"    pub {}: Vec<Color>,"#,
                    snake_case(list_name)
                )?;
            }
        }
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
    for (_idx, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with("\"") {
            continue;
        }
        if line.contains("\": [") {
            let (list_name, _) = line
                .split_once("\": ")
                .ok_or_else(|| anyhow::anyhow!("Failed to parse list name"))?;
            if let Some(list_name) = list_name.strip_prefix("\"") {
                let title_cased = title_case_list_name(&list_name);
                let snake_cased = snake_case(&list_name);
                writeln!(
                    &mut models_file,
                    r#"            (Lists::{title_cased}, data.{snake_cased}),"#
                )?;
            }
        }
    }
    writeln!(&mut models_file, r#"        ]))"#)?;
    writeln!(&mut models_file, r#"    }}"#)?;
    writeln!(&mut models_file, r#"}}"#)?;

    std::fs::write(&dest_path, s)?;

    Ok(())
}

fn title_case(s: &str) -> String {
    let mut s = s.to_string();
    format!("{}{s}", s.remove(0).to_uppercase())
}

fn title_case_list_name(list_name: &str) -> String {
    match list_name {
        s if s.starts_with("mlmc") => {
            let (part1, part2) = s.rsplit_once("_").unwrap();
            let part1 = title_case(part1);
            let part2 = title_case(part2);
            format!("{part1}{part2}")
        }
        _ => title_case(&list_name),
    }
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
