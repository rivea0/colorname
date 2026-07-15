use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::io::Write as IoWrite;
use std::path::Path;

use crate::command::{OutputFormats, SourceList};

use colorname::models::{Color, ColorItem, ColorNameLists};

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn read_lists_from_file<P: AsRef<Path>>(path: P) -> Result<ColorNameLists, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lst = serde_json::from_reader(reader)?;

    Ok(lst)
}

pub fn write_to_file(file_type: OutputFormats, result: &Vec<Color>) {
    if file_type == OutputFormats::Html {
        println!("Creating html file...");

        let mut rows = String::new();

        for color in result.iter() {
            let name = escape_html(&color.name);
            let hex = escape_html(&color.hex);
            write!(
                rows,
               "    <div class=\"color-container\">\n      <div style=\"background-color: {hex};\"></div>\n      <p>{name}</p>\n      <p>{hex}</p>\n    </div>\n"
            ).unwrap();
        }

        let html = format!(
            r#"<!doctype html>
<html lang="en">
  <head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>colors</title>
  </head>
  <style>
    body {{
      display: flex;
      flex-direction: column;
      align-items: center;
    }}
    .color-container > div {{
      width: 200px;
      height: 150px;
    }}
  </style>
  <body>
{rows}  </body>
</html>"#
        );

        // Will override
        let mut f = File::create("./colorname-output.html").unwrap();
        f.write_all(html.as_bytes()).unwrap();
    }
    if file_type == OutputFormats::Json {
        println!("Creating json file...");
        // create json string, populate fields from result
        // write string to file
    }
    if file_type == OutputFormats::Csv {
        println!("Creating csv file...");
        // create csv string, populate fields from result
        // write string to file
    }
}

pub fn get_colors<T>(color_name: &str, list: &Vec<T>, with_info: bool) -> Vec<Color>
where
    T: ColorItem + std::fmt::Debug,
{
    let mut v = vec![];
    for color in list {
        if color.name().to_lowercase().contains(color_name) {
            if with_info {
                v.push(Color {
                    name: color.name().to_string(),
                    hex: color.hex().to_string(),
                    meta: color.meta().clone(),
                });
            } else {
                v.push(Color {
                    name: color.name().to_string(),
                    hex: color.hex().to_string(),
                    meta: None,
                });
            }
        }
    }
    v
}

pub fn find_in_lists(pattern: &str, source_list: &SourceList, with_info: bool) -> Vec<Color> {
    let lists = read_lists_from_file("./data/colorlists.json").unwrap();

    let mut result = vec![];

    if *source_list == SourceList::default() {
        println!("Searching {pattern} in wikipedia..."); // default
        result.push(get_colors(pattern, &lists.wikipedia, with_info));
    } else {
        if source_list.wikipedia {
            result.push(get_colors(pattern, &lists.wikipedia, with_info));
        }
        if source_list.basic {
            result.push(get_colors(pattern, &lists.basic, with_info));
        }
        if source_list.html {
            result.push(get_colors(pattern, &lists.html, with_info));
        }
        if source_list.french {
            result.push(get_colors(pattern, &lists.french, with_info));
        }
        if source_list.spanish {
            result.push(get_colors(pattern, &lists.spanish, with_info));
        }
        if source_list.german {
            result.push(get_colors(pattern, &lists.german, with_info));
        }
        if source_list.ridgway {
            result.push(get_colors(pattern, &lists.ridgway, with_info));
        }
        if source_list.risograph {
            result.push(get_colors(pattern, &lists.risograph, with_info));
        }
        if source_list.hindi {
            result.push(get_colors(pattern, &lists.hindi, with_info));
        }
        if source_list.chinese_traditional {
            result.push(get_colors(pattern, &lists.chinese_traditional, with_info));
        }
        if source_list.japanese_traditional {
            result.push(get_colors(pattern, &lists.japanese_traditional, with_info));
        }
        if source_list.le_corbusier {
            result.push(get_colors(pattern, &lists.le_corbusier, with_info));
        }
        if source_list.nbs_iscc {
            result.push(get_colors(pattern, &lists.nbs_iscc, with_info));
        }
        if source_list.ntc {
            result.push(get_colors(pattern, &lists.ntc, with_info));
        }
        if source_list.osxcrayons {
            result.push(get_colors(pattern, &lists.osxcrayons, with_info));
        }
        if source_list.ral {
            result.push(get_colors(pattern, &lists.ral, with_info));
        }
        if source_list.sanzo_wada_i {
            result.push(get_colors(pattern, &lists.sanzo_wada_i, with_info));
        }
        if source_list.thesaurus {
            result.push(get_colors(pattern, &lists.thesaurus, with_info));
        }
        if source_list.werner {
            result.push(get_colors(pattern, &lists.werner, with_info));
        }
        if source_list.windows {
            result.push(get_colors(pattern, &lists.windows, with_info));
        }
        if source_list.x11 {
            result.push(get_colors(pattern, &lists.x11, with_info));
        }
        if source_list.xkcd {
            result.push(get_colors(pattern, &lists.xkcd, with_info));
        }
        if source_list.mlmc_korean {
            result.push(get_colors(pattern, &lists.mlmc_korean, with_info));
        }
        if source_list.mlmc_english {
            result.push(get_colors(pattern, &lists.mlmc_english, with_info));
        }
        if source_list.mlmc_chinese {
            result.push(get_colors(pattern, &lists.mlmc_chinese, with_info));
        }
        if source_list.mlmc_russian {
            result.push(get_colors(pattern, &lists.mlmc_russian, with_info));
        }
        if source_list.mlmc_german {
            result.push(get_colors(pattern, &lists.mlmc_german, with_info));
        }
        if source_list.mlmc_spanish {
            result.push(get_colors(pattern, &lists.mlmc_spanish, with_info));
        }
        if source_list.mlmc_finnish {
            result.push(get_colors(pattern, &lists.mlmc_finnish, with_info));
        }
        if source_list.mlmc_dutch {
            result.push(get_colors(pattern, &lists.mlmc_dutch, with_info));
        }
        if source_list.mlmc_portuguese {
            result.push(get_colors(pattern, &lists.mlmc_portuguese, with_info));
        }
        if source_list.mlmc_romanian {
            result.push(get_colors(pattern, &lists.mlmc_romanian, with_info));
        }
        if source_list.mlmc_swedish {
            result.push(get_colors(pattern, &lists.mlmc_swedish, with_info));
        }
        if source_list.mlmc_polish {
            result.push(get_colors(pattern, &lists.mlmc_polish, with_info));
        }
        if source_list.mlmc_persian {
            result.push(get_colors(pattern, &lists.mlmc_persian, with_info));
        }
        if source_list.mlmc_french {
            result.push(get_colors(pattern, &lists.mlmc_french, with_info));
        }
    }
    result.into_iter().flatten().collect::<Vec<Color>>()
}
