use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_to_command_file(
    command_file_path: impl AsRef<Path>,
    lang_aliases: &HashMap<&str, &str>,
    list_names: &Vec<&str>,
) -> Result<()> {
    let command_file_path = command_file_path.as_ref();
    let mut command_file = File::options().append(true).open(command_file_path)?;
    let desc_hash_map = extract_from_descriptions_file(list_names, "description")?;

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
        writeln!(
            &mut command_file,
            r#"    /// {}"#,
            desc_hash_map.get(&n).unwrap()
        )?;
        if let Some((k, _)) = lang_aliases.iter().find(|(_, v)| n.starts_with(*v)) {
            writeln!(
                &mut command_file,
                r#"    #[arg(long, visible_alias = "{k}")]"#
            )?;
        } else if let Some((k, _)) = lang_aliases
            .iter()
            .find(|(_, v)| n.starts_with("mlmc") && n.contains(*v))
        {
            writeln!(
                &mut command_file,
                r#"    #[arg(long, visible_alias = "mlmc-{k}")]"#
            )?;
        } else {
            writeln!(&mut command_file, r#"    #[arg(long)]"#)?;
        }
        writeln!(&mut command_file, r#"    pub {n}: bool,"#)?;
    }

    for v in ["all_en", "all_de", "all_fr", "all_es", "all_zh"] {
        let (_, alias) = v.split_once('_').unwrap();
        let lang = lang_aliases.get(alias).unwrap();
        writeln!(
            &mut command_file,
            r#"    /// Search in all {} language lists"#,
            title_case(lang)
        )?;
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

    Ok(())
}

pub fn write_to_models_file(
    models_file_path: impl AsRef<Path>,
    list_names: &Vec<&str>,
) -> Result<()> {
    let models_file_path = models_file_path.as_ref();
    let mut models_file = File::options().append(true).open(models_file_path)?;

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

    Ok(())
}

pub fn write_to_readme_file(
    s: String,
    readme_file_path: impl AsRef<Path>,
    list_names: &Vec<&str>,
    lang_aliases: &HashMap<&str, &str>,
) -> Result<()> {
    let readme_file_path = readme_file_path.as_ref();
    let desc_hash_map = extract_from_descriptions_file(list_names, "description")?;
    let source_hash_map = extract_from_descriptions_file(list_names, "source")?;
    let start_marker = "<!-- start options -->";
    let end_marker = "<!-- end options -->";

    let start = s.find(start_marker).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "start marker not found")
    })?;

    let insert_start = start + start_marker.len();
    let mut s_to_add = "### Options\n\n#### Lists\n\n".to_string();

    for n in list_names.iter() {
        let to_add = if let Some((k, _)) = lang_aliases.iter().find(|(_, v)| n.starts_with(*v)) {
            format!(
                "##### `--{}` (alias = `--{}`)\n",
                snake_case(n).replace("_", "-"),
                k
            )
        } else if let Some((k, _)) = lang_aliases
            .iter()
            .find(|(_, v)| n.starts_with("mlmc") && n.contains(*v))
        {
            format!(
                "##### `--{}` (alias = `--mlmc-{}`)\n",
                snake_case(n).replace("_", "-"),
                k
            )
        } else {
            format!("##### `--{}`\n", snake_case(n).replace("_", "-"))
        };

        s_to_add = format!("{s_to_add}{to_add}\n");
        let n = snake_case(n);
        let description = desc_hash_map.get(&n).unwrap();
        let source_link = source_hash_map.get(&n).unwrap();

        s_to_add = format!("{s_to_add}> \"{description}\"\n\n");
        s_to_add = format!("{s_to_add}Source: {source_link}\n\n");
    }

    let rel_end = s[insert_start..]
        .find(end_marker)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "end marker not found"))?;

    let insert_end = insert_start + rel_end;

    let updated = format!(
        "{}{}{}{}{}",
        &s[..insert_start],
        "\n",
        s_to_add,
        "\n",
        &s[insert_end..],
    );

    std::fs::write(readme_file_path, updated)?;

    Ok(())
}

pub fn snake_case(s: &str) -> String {
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

fn extract_from_descriptions_file(
    list_names: &Vec<&str>,
    key: &str,
) -> Result<HashMap<String, String>> {
    let descriptions_file = include_str!("./data/descriptions.json");
    let mut h: HashMap<_, _> = HashMap::new();
    let desc_lines = descriptions_file.lines().collect::<Vec<_>>();

    // the line offsets from the key (list_name)
    let line_offsets = HashMap::from([
        ("title", 1),
        ("description", 2),
        ("source", 3),
        ("key", 4),
        ("license", 5),
    ]);

    for n in list_names.iter() {
        for (idx, line) in desc_lines.iter().enumerate() {
            let line = line.trim();
            if (!line.is_empty() || !line.starts_with("\"")) && line.contains("\": {") {
                let (nme, _) = line.split_once("\": ").unwrap();
                let nme = nme.strip_prefix("\"").unwrap();
                let snake_cased = snake_case(nme);
                if snake_cased.as_str() == snake_case(n).as_str()
                    && let Some(desc_line) = desc_lines.get(idx + line_offsets.get(key).unwrap())
                    && desc_line.contains(key)
                {
                    let (_, val) = desc_line.split_once("\": \"").unwrap();
                    let val = val.strip_suffix("\",").unwrap_or(val);
                    h.insert(snake_cased, val.to_string());
                }
            }
        }
    }
    Ok(h)
}
