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
        let n = title_case_list_name(&n);
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
        if is_camel_case(&n) {
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
        let title_cased = title_case_list_name(&n);
        let snake_cased = snake_case(&n);
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
        let n = snake_case(&n);
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

    std::fs::write(&dest_path, lists)?;
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

fn is_camel_case(s: &str) -> bool {
    s != s.to_lowercase() && s != s.to_uppercase() && !s.contains('_')
}
