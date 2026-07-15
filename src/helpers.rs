use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::command::SourceList;

use colorname::models::{Color, ColorItem, ColorNameLists};

pub fn read_lists_from_file<P: AsRef<Path>>(path: P) -> Result<ColorNameLists, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lst = serde_json::from_reader(reader)?;

    Ok(lst)
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
