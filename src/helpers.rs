use anyhow::{Context, Result, bail};
use comfy_table::{Attribute, Cell, Color, Table, presets::UTF8_FULL};
use indexmap::{IndexMap, IndexSet};
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;

use crate::command::OutputFormats;
use colorname::models::{Color as CrateColor, Lists};

pub fn write_to_file(
    file_type: OutputFormats,
    result: &IndexMap<Lists, Vec<CrateColor>>,
    file_path: impl AsRef<Path>,
) -> Result<()> {
    let file_path = file_path.as_ref();
    match file_type {
        OutputFormats::Html => {
            println!("Creating html file...");
            if !result.is_empty() {
                let html = create_html_string(result).context("Could not create HTML string")?;
                let mut f = File::create(file_path)?;
                f.write_all(html.as_bytes())?;
            }
        }
        OutputFormats::Json => {
            println!("Creating json file...");
            if !result.is_empty() {
                // Will override
                let f = File::create(file_path)?;
                serde_json::to_writer(f, result)?;
            }
        }

        OutputFormats::Csv => {
            println!("Creating csv file...");
            let mut wtr = csv::Writer::from_path(file_path)?;

            let default_headers = ["list", "name", "hex"];
            let mut headers = default_headers.into_iter().collect::<IndexSet<_>>();

            for metadata_key in result
                .values()
                .flat_map(|colors| colors.iter())
                .filter_map(|color| color.meta.as_ref())
                .flat_map(|metadata| metadata.keys())
                .map(|key| key.as_str())
            {
                headers.insert(metadata_key);
            }

            wtr.write_record(&headers)?;

            let meta_headers = &headers[default_headers.len()..];

            for (list_name, colors) in result.iter() {
                for color in colors {
                    let mut values = csv::StringRecord::new();
                    values.push_field(&list_name.to_string());
                    values.push_field(&color.name);
                    values.push_field(&color.hex);
                    for header in meta_headers {
                        // Add empty fields if a metadata header doesn't exist in a color.
                        // For colors that don't have metadata, fill all fields with
                        // empty values for all metadata headers
                        let header_str = color
                            .meta
                            .as_ref()
                            .and_then(|metadata| metadata.get(*header))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        values.push_field(header_str);
                    }
                    wtr.write_record(&values)?;
                }
            }
        }
    }

    Ok(())
}

fn create_html_string(result: &IndexMap<Lists, Vec<CrateColor>>) -> Result<String> {
    let mut rows = String::new();
    for (list_name, colors) in result.iter() {
        writeln!(rows, "    <h1>List: {list_name}</h1>")?;
        for color in colors.iter() {
            let name = escape_html(&color.name);
            let hex = escape_html(&color.hex);
            writeln!(
                rows,
                "    <div class=\"color-container\">\n      <div style=\"background-color: {hex};\"></div>\n      <p>{name}</p>\n      <p>{hex}</p>"
            )?;
            if let Some(metadata) = &color.meta {
                for (k, v) in metadata {
                    writeln!(rows, "      <p>{k}: {v}</p>")?;
                }
            }
            writeln!(rows, "    </div>")?;
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
    body {{
      padding: 1rem;
    }}
    h1 {{
      text-align: center;
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

pub fn get_rgb_value(hex_str: &str) -> Result<[u8; 3]> {
    if hex_str.chars().count() != 7 {
        bail!(
            "Expected 7 characters (such as #RRGGBB), got {}",
            hex_str.chars().count()
        );
    }

    let Some('#') = hex_str.chars().next() else {
        bail!("Expected string to start with '#'");
    };

    // Remove the hash symbol
    let hex_str = &hex_str[1..];
    if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("Expected only hex digits after '#'");
    }

    let r = u8::from_str_radix(&hex_str[..2], 16)?;
    let g = u8::from_str_radix(&hex_str[2..4], 16)?;
    let b = u8::from_str_radix(&hex_str[4..], 16)?;

    Ok([r, g, b])
}

pub fn create_table(values: Vec<CrateColor>, styled: bool, with_info: bool) -> Result<Table> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL.with_solid_inner_borders());
    set_table_header(&mut table, styled);
    set_table_rows(&mut table, values, styled, with_info)?;

    Ok(table)
}

pub fn set_table_header(table: &mut Table, styled: bool) {
    if styled {
        table.set_header(vec![
            Cell::new("name").add_attribute(Attribute::Bold),
            Cell::new("hex").add_attribute(Attribute::Bold),
        ]);
    } else {
        table.set_header(vec!["name", "hex"]);
    }
}

pub fn set_table_rows(
    table: &mut Table,
    values: Vec<CrateColor>,
    styled: bool,
    with_info: bool,
) -> Result<()> {
    for value in values {
        if !styled {
            let mut v = vec![vec![value.name, value.hex]];
            if with_info && let Some(meta_values) = value.meta {
                for (key, val) in meta_values {
                    v.push(vec![format!("{key}: {val}")]);
                }
            }
            table.add_rows(v);
        } else {
            let rgb_vals = get_rgb_value(&value.hex)?;
            let mut v = vec![vec![
                Cell::new(value.name).fg(Color::Rgb {
                    r: rgb_vals[0],
                    g: rgb_vals[1],
                    b: rgb_vals[2],
                }),
                Cell::new(value.hex).fg(Color::Rgb {
                    r: rgb_vals[0],
                    g: rgb_vals[1],
                    b: rgb_vals[2],
                }),
                Cell::new("    ".to_string()).bg(Color::Rgb {
                    r: rgb_vals[0],
                    g: rgb_vals[1],
                    b: rgb_vals[2],
                }),
            ]];
            if with_info && let Some(meta_values) = value.meta {
                for (key, val) in meta_values {
                    v.push(vec![Cell::new(format!("{key}: {val}"))]);
                }
            }
            table.add_rows(v);
        }
    }

    Ok(())
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs};

    use super::*;

    #[test]
    fn write_to_file_creates_html() -> Result<()> {
        let expected_text = format!(
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
    body {{
      padding: 1rem;
    }}
    h1 {{
      text-align: center;
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
    <h1>List: Basic</h1>
    <div class="color-container">
      <div style="background-color: #000000;"></div>
      <p>black</p>
      <p>#000000</p>
    </div>
  </body>
</html>"#
        );

        assert_output(OutputFormats::Html, "colors_output.html", &expected_text)?;

        Ok(())
    }

    #[test]
    fn write_to_file_creates_html_with_metadata() -> Result<()> {
        let expected_text = format!(
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
    body {{
      padding: 1rem;
    }}
    h1 {{
      text-align: center;
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
    <h1>List: Wikipedia</h1>
    <div class="color-container">
      <div style="background-color: #000000;"></div>
      <p>black</p>
      <p>#000000</p>
      <p>link: https://en.wikipedia.org/wiki/Black</p>
    </div>
  </body>
</html>"#
        );

        assert_output_with_metadata(OutputFormats::Html, "colorname_output.html", &expected_text)?;

        Ok(())
    }

    #[test]
    fn write_to_file_creates_json() -> Result<()> {
        let expected_text = format!(r##"{{"Basic":[{{"name":"black","hex":"#000000"}}]}}"##);

        assert_output(OutputFormats::Json, "colorname_output.json", &expected_text)?;

        Ok(())
    }

    #[test]
    fn write_to_file_creates_json_with_metadata() -> Result<()> {
        let expected_text = format!(
            r##"{{"Wikipedia":[{{"name":"black","hex":"#000000","meta":{{"link":"https://en.wikipedia.org/wiki/Black"}}}}]}}"##
        );

        assert_output_with_metadata(OutputFormats::Json, "colorname_output.json", &expected_text)?;

        Ok(())
    }

    #[test]
    fn write_to_file_creates_csv() -> Result<()> {
        let expected_text = "list,name,hex\nBasic,black,#000000\n";

        assert_output(OutputFormats::Csv, "colorname_output.csv", expected_text)?;

        Ok(())
    }

    #[test]
    fn write_to_file_creates_csv_with_metadata() -> Result<()> {
        let expected_text =
            "list,name,hex,link\nWikipedia,black,#000000,https://en.wikipedia.org/wiki/Black\n";

        assert_output_with_metadata(OutputFormats::Csv, "colorname_output.csv", expected_text)?;

        Ok(())
    }

    #[test]
    fn write_to_file_creates_csv_with_empty_metadata_fields() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let p = dir.path().join("colorname_output.csv");

        let res = IndexMap::from([
            (
                Lists::Wikipedia,
                vec![CrateColor {
                    name: "black".to_string(),
                    hex: "#000000".to_string(),
                    meta: Some(BTreeMap::from([(
                        "link".into(),
                        "https://en.wikipedia.org/wiki/Black".into(),
                    )])),
                }],
            ),
            (
                Lists::Thesaurus,
                vec![CrateColor {
                    name: "Black".to_string(),
                    hex: "#000000".to_string(),
                    meta: Some(BTreeMap::from([("category".into(), "black".into())])),
                }],
            ),
            (
                Lists::Basic,
                vec![CrateColor {
                    name: "black".to_string(),
                    hex: "#000000".to_string(),
                    meta: None,
                }],
            ),
        ]);

        let expected_text = "list,name,hex,link,category\nWikipedia,black,#000000,https://en.wikipedia.org/wiki/Black,\nThesaurus,Black,#000000,,black\nBasic,black,#000000,,\n";

        write_to_file(OutputFormats::Csv, &res, &p)?;
        let text = fs::read_to_string(p)?;

        assert_eq!(text, expected_text);

        Ok(())
    }

    #[test]
    fn get_rgb_value_works_for_correct_input() -> Result<()> {
        let result = get_rgb_value("#000000")?;
        let expected = [0, 0, 0];

        assert_eq!(result, expected);

        let result = get_rgb_value("#ffffff")?;
        let expected = [255, 255, 255];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn get_rgb_value_bails_when_given_incorrect_number_of_chars() -> Result<()> {
        let result = get_rgb_value("000000").unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected 7 characters (such as #RRGGBB), got 6")
        );

        Ok(())
    }

    #[test]
    fn get_rgb_value_bails_when_input_starts_with_incorrect_char() -> Result<()> {
        let result = get_rgb_value("+000000").unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected string to start with '#'")
        );

        Ok(())
    }

    #[test]
    fn get_rgb_value_bails_when_given_input_includes_non_hex_digits() -> Result<()> {
        let result = get_rgb_value("#0000gg").unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected only hex digits after '#'")
        );

        Ok(())
    }

    fn assert_output(output_format: OutputFormats, file_name: &str, expected: &str) -> Result<()> {
        let dir = tempfile::tempdir()?;
        let p = dir.path().join(file_name);

        let res = IndexMap::from([(
            Lists::Basic,
            vec![CrateColor {
                name: "black".to_string(),
                hex: "#000000".to_string(),
                meta: None,
            }],
        )]);

        write_to_file(output_format, &res, &p)?;
        let text = fs::read_to_string(p)?;

        assert_eq!(text, expected);

        Ok(())
    }

    fn assert_output_with_metadata(
        output_format: OutputFormats,
        file_name: &str,
        expected: &str,
    ) -> Result<()> {
        let dir = tempfile::tempdir()?;
        let p = dir.path().join(file_name);

        let res = IndexMap::from([(
            Lists::Wikipedia,
            vec![CrateColor {
                name: "black".to_string(),
                hex: "#000000".to_string(),
                meta: Some(BTreeMap::from([(
                    "link".into(),
                    "https://en.wikipedia.org/wiki/Black".into(),
                )])),
            }],
        )]);

        write_to_file(output_format, &res, &p)?;
        let text = fs::read_to_string(p)?;

        assert_eq!(text, expected);

        Ok(())
    }
}
