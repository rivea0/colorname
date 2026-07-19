use anyhow::{Context, Result};
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
        // if !result.is_empty() {
        //     let j = json!({"result": result});
        //     // Will override
        //     let mut f = File::create("./colorname-output.json")?;
        //     f.write_all(j.to_string().as_bytes())?;
        // }
    }
    if file_type == OutputFormats::Csv {
        println!("Creating csv file...");
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

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
