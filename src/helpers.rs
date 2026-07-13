use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::command::SourceList;

use colorname::models::{BaseColor, Color, ColorNameLists};

pub fn read_lists_from_file<P: AsRef<Path>>(path: P) -> Result<ColorNameLists, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lst = serde_json::from_reader(reader)?;

    Ok(lst)
}

pub fn get_colors<T>(color_name: &str, list: &Vec<T>) -> Vec<Color>
where
    T: BaseColor + std::fmt::Debug,
{
    let mut v = vec![];
    for color in list {
        if color.name().to_lowercase().contains(color_name) {
            v.push(Color {
                name: color.name().to_string(),
                hex: color.hex().to_string(),
            });
        }
    }
    v
}

pub fn find_in_lists(pattern: &str, source_list: &SourceList) -> Vec<Color> {
    let lists = read_lists_from_file("./data/colorlists.json").unwrap();

    let mut result = vec![];

    if *source_list == SourceList::default() {
        println!("Searching {pattern} in wikipedia..."); // default
        result.push(get_colors(pattern, &lists.wikipedia));
    } else {
        if source_list.wikipedia {
            result.push(get_colors(pattern, &lists.wikipedia));
        }
        if source_list.basic {
            result.push(get_colors(pattern, &lists.basic));
        }
        if source_list.html {
            result.push(get_colors(pattern, &lists.html));
        }
        if source_list.french {
            result.push(get_colors(pattern, &lists.french));
        }
        if source_list.spanish {
            result.push(get_colors(pattern, &lists.spanish));
        }
        if source_list.german {
            result.push(get_colors(pattern, &lists.german));
        }
        if source_list.ridgway {
            result.push(get_colors(pattern, &lists.ridgway));
        }
        if source_list.risograph {
            result.push(get_colors(pattern, &lists.risograph));
        }
        if source_list.hindi {
            result.push(get_colors(pattern, &lists.hindi));
        }
        if source_list.chinese_traditional {
            result.push(get_colors(pattern, &lists.chinese_traditional));
        }
        if source_list.japanese_traditional {
            result.push(get_colors(pattern, &lists.japanese_traditional));
        }
        if source_list.le_corbusier {
            result.push(get_colors(pattern, &lists.le_corbusier));
        }
        if source_list.nbs_iscc {
            result.push(get_colors(pattern, &lists.nbs_iscc));
        }
        if source_list.ntc {
            result.push(get_colors(pattern, &lists.ntc));
        }
        if source_list.osxcrayons {
            result.push(get_colors(pattern, &lists.osxcrayons));
        }
        if source_list.ral {
            result.push(get_colors(pattern, &lists.ral));
        }
        if source_list.sanzo_wada_i {
            result.push(get_colors(pattern, &lists.sanzo_wada_i));
        }
        if source_list.thesaurus {
            result.push(get_colors(pattern, &lists.thesaurus));
        }
        if source_list.werner {
            result.push(get_colors(pattern, &lists.werner));
        }
        if source_list.windows {
            result.push(get_colors(pattern, &lists.windows));
        }
        if source_list.x11 {
            result.push(get_colors(pattern, &lists.x11));
        }
        if source_list.xkcd {
            result.push(get_colors(pattern, &lists.xkcd));
        }
        if source_list.mlmc_korean {
            result.push(get_colors(pattern, &lists.mlmc_korean));
        }
        if source_list.mlmc_english {
            result.push(get_colors(pattern, &lists.mlmc_english));
        }
        if source_list.mlmc_chinese {
            result.push(get_colors(pattern, &lists.mlmc_chinese));
        }
        if source_list.mlmc_russian {
            result.push(get_colors(pattern, &lists.mlmc_russian));
        }
        if source_list.mlmc_german {
            result.push(get_colors(pattern, &lists.mlmc_german));
        }
        if source_list.mlmc_spanish {
            result.push(get_colors(pattern, &lists.mlmc_spanish));
        }
        if source_list.mlmc_finnish {
            result.push(get_colors(pattern, &lists.mlmc_finnish));
        }
        if source_list.mlmc_dutch {
            result.push(get_colors(pattern, &lists.mlmc_dutch));
        }
        if source_list.mlmc_portuguese {
            result.push(get_colors(pattern, &lists.mlmc_portuguese));
        }
        if source_list.mlmc_romanian {
            result.push(get_colors(pattern, &lists.mlmc_romanian));
        }
        if source_list.mlmc_swedish {
            result.push(get_colors(pattern, &lists.mlmc_swedish));
        }
        if source_list.mlmc_polish {
            result.push(get_colors(pattern, &lists.mlmc_polish));
        }
        if source_list.mlmc_persian {
            result.push(get_colors(pattern, &lists.mlmc_persian));
        }
        if source_list.mlmc_french {
            result.push(get_colors(pattern, &lists.mlmc_french));
        }
    }
    result.into_iter().flatten().collect::<Vec<Color>>()
}
