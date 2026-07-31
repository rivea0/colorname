use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IoWrite;

use crate::command::OutputFormats;
use colorname::models::{Color, Lists};

pub fn write_to_file(file_type: OutputFormats, result: &HashMap<Lists, Vec<Color>>) -> Result<()> {
    if file_type == OutputFormats::Html {
        println!("Creating html file...");
        if !result.is_empty() {
            let html = create_html_string(result)
                .with_context(|| format!("Could not create HTML string"))?;
            let mut f = File::create("./colorname-output.html")?;
            f.write_all(html.as_bytes())?;
        }
    }
    if file_type == OutputFormats::Json {
        println!("Creating json file...");
        if !result.is_empty() {
            let j = json!({"result": result});
            // Will override
            let mut f = File::create("./colorname-output.json")?;
            f.write_all(j.to_string().as_bytes())?;
        }
    }
    if file_type == OutputFormats::Csv {
        println!("Creating csv file...");
        File::create("./colorname-output.csv")?;
        let mut wtr = csv::Writer::from_path("./colorname-output.csv")?;
        wtr.write_record(["Name", "Hex", "List"])?;
        for (list_name, values) in result {
            for value in values {
                wtr.write_record([&value.name, &value.hex, &list_name.to_string()])?;
            }
        }
    }

    Ok(())
}

pub fn create_html_string(result: &HashMap<Lists, Vec<Color>>) -> Result<String> {
    let mut rows = String::new();
    for (list_name, colors) in result.iter() {
        write!(rows, "    <h1>In list: {list_name:?}</h1>")?;
        for color in colors.iter() {
            let name = escape_html(&color.name);
            let hex = escape_html(&color.hex);
            write!(
                rows,
                "    <div class=\"color-container\">\n      <div style=\"background-color: {hex};\"></div>\n      <p>{name}</p>\n      <p>{hex}</p>\n    </div>\n"
            )?;
        }
    }

    Ok(format!(
        r#"<!doctype html>
<html lang="en">
  <head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>colors</title>
  </head>
  <style>
    * {{
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }}
    .color-container {{
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1rem;
      margin: 4rem 0;
      font-family: sans-serif;
    }}
    .color-container > div {{
      width: calc(100vw - 25%);
      height: 8rem;
      border-radius: 1rem;
    }}        
  </style>
  <body>
{rows}  </body>
</html>"#
    ))
}

pub fn get_rgb(hex_str: &str) -> Result<[u8; 3]> {
    if hex_str.chars().count() != 7 {
        eprintln!("Character count is wrong");
    }
    let first_char = hex_str.chars().nth(0).unwrap();
    if first_char != '#' {
        eprintln!("Not starting with hash");
    }
    // Remove the hash symbol
    let hex_str = &hex_str[1..];
    if !hex_str.chars().all(|c| c.is_ascii_alphanumeric()) {
        eprintln!("Wrong");
    }
    let parts = [&hex_str[..2], &hex_str[2..4], &hex_str[4..]];

    Ok(parts.map(|part| u8::from_str_radix(part, 16).unwrap()))
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
