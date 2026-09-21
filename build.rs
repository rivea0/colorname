use anyhow::Result;
use std::collections::HashMap;
use std::env::var_os;
use std::path::Path;

mod build_helpers;

use build_helpers::{write_to_command_file, write_to_models_file, write_to_readme_file};

fn main() -> Result<()> {
    let lists = include_str!("./data/colorlists.json");
    let command_file_path = "./src/command.rs";
    let models_file_path = "./src/models.rs";
    let readme_file_path = "./README.md";
    let out_dir = var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("colorlists.json");

    let mut list_names = lists
        .lines()
        .map(|line| line.trim())
        .filter(|line| (!line.is_empty() || !line.starts_with("\"")) && line.contains("\": ["))
        .filter_map(|line| line.split_once("\": "))
        .filter_map(|(list_name, _)| list_name.strip_prefix("\""))
        .collect::<Vec<_>>();
    list_names.sort();

    let lang_aliases = HashMap::from([
        ("de", "german"),
        ("fr", "french"),
        ("ja", "japanese"),
        ("zh", "chinese"),
        ("hi", "hindi"),
        ("es", "spanish"),
        ("en", "english"),
        ("nl", "dutch"),
        ("fi", "finnish"),
        ("ko", "korean"),
        ("fa", "persian"),
        ("pl", "polish"),
        ("pt", "portuguese"),
        ("ro", "romanian"),
        ("ru", "russian"),
        ("sv", "swedish"),
    ]);

    let s = std::fs::read_to_string(command_file_path)?;
    let mut update_command_file = true;
    for line in s.lines() {
        if line.contains("pub struct SourceList") {
            update_command_file = false;
        }
    }

    if update_command_file {
        write_to_command_file(command_file_path, &lang_aliases, &list_names)?;
    }

    let s = std::fs::read_to_string(models_file_path)?;
    let mut update_models_file = true;
    for line in s.lines() {
        if line.contains("pub enum Lists") {
            update_models_file = false;
        }
    }

    if update_models_file {
        write_to_models_file(models_file_path, &list_names)?;
    }

    let s = std::fs::read_to_string(readme_file_path)?;
    let mut update_readme_file = true;

    for line in s.lines() {
        if line.contains("### Options") {
            update_readme_file = false;
        }
    }

    if update_readme_file {
        write_to_readme_file(s, readme_file_path, &list_names, &lang_aliases)?;
    }

    std::fs::write(&dest_path, lists)?;
    Ok(())
}
