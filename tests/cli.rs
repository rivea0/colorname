use anyhow::Result;
use assert_cmd::{Command, pkg_name};
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn it_prints_color_from_the_default_list() {
    let includes_default_list = predicate::str::contains("List: Wikipedia");
    let includes_color = predicate::str::contains("#ff7518");
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .arg("pumpkin")
        .assert()
        .success()
        .stdout(includes_default_list.and(includes_color));
}

#[test]
fn it_prints_not_found_when_color_does_not_exist() {
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .arg("nope")
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn it_includes_multiple_english_lists_with_all_en() {
    let includes_all = predicate::str::contains("List: MlmcEnglish")
        .and(predicate::str::contains("List: Ntc"))
        .and(predicate::str::contains("List: Risograph"))
        .and(predicate::str::contains("List: Wikipedia"))
        .and(predicate::str::contains("List: Xkcd"));
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .args(["pumpkin", "--all-en"])
        .assert()
        .success()
        .stdout(includes_all);
}

#[test]
fn it_prints_color_in_german() {
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .args(["nacht", "--de"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("List: German")
                .and(predicate::str::contains("Mitternachtsblau"))
                .and(predicate::str::contains("#191970")),
        );
}

#[test]
fn it_includes_multiple_german_lists_with_all_de() {
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .args(["schwarz", "--all-de"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("List: German")
                .and(predicate::str::contains("List: MlmcGerman")),
        );
}

#[test]
fn it_prints_color_in_french() {
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .args(["nuit", "--fr"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("List: French")
                .and(predicate::str::contains("Bleu Nuit"))
                .and(predicate::str::contains("#0f056b")),
        );
}

#[test]
fn it_includes_multiple_french_lists_with_all_fr() {
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .args(["noir", "--all-fr"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("List: French")
                .and(predicate::str::contains("List: LeCorbusier"))
                .and(predicate::str::contains("List: MlmcFrench")),
        );
}

#[test]
fn it_does_not_show_output_when_the_quiet_option_is_passed() {
    Command::cargo_bin(pkg_name!())
        .unwrap()
        .args([
            "sky",
            "--output",
            "html",
            "--file-path",
            "./result.html",
            "--quiet",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("List: Wikipedia").not());
}

#[test]
fn creates_html_file_when_passed_output_format() -> Result<()> {
    let output_file = NamedTempFile::new()?;

    let expected_text = r#"<!doctype html>
<html lang="en">
  <head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>colors</title>
  </head>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    body {
      padding: 1rem;
    }
    h1 {
      text-align: center;
    }
    .color-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1rem;
      margin: 4rem 0;
      font-family: sans-serif;
    }
    .color-container > div {
      width: calc(100vw - 25%);
      height: 8rem;
      border-radius: 1rem;
    }
  </style>
  <body>
    <h1>List: Wikipedia</h1>
    <div class="color-container">
      <div style="background-color: #ff7518;"></div>
      <p>Pumpkin</p>
      <p>#ff7518</p>
    </div>
  </body>
</html>"#;

    Command::cargo_bin(pkg_name!())
        .unwrap()
        .arg("pumpkin")
        .arg("--output")
        .arg("html")
        .arg("--file-path")
        .arg(output_file.path())
        .arg("--quiet")
        .assert()
        .success()
        .stdout("Creating html file...\n");

    let text = std::fs::read_to_string(output_file.path())?;
    assert_eq!(text, expected_text);

    Ok(())
}

#[test]
fn creates_json_file_when_passed_output_format() -> Result<()> {
    let output_file = NamedTempFile::new()?;

    let expected_text = format!(r##"{{"Wikipedia":[{{"name":"Pumpkin","hex":"#ff7518"}}]}}"##);

    Command::cargo_bin(pkg_name!())
        .unwrap()
        .arg("pumpkin")
        .arg("--output")
        .arg("json")
        .arg("--file-path")
        .arg(output_file.path())
        .arg("--quiet")
        .assert()
        .success()
        .stdout("Creating json file...\n");

    let text = std::fs::read_to_string(output_file.path())?;
    assert_eq!(text, expected_text);

    Ok(())
}

#[test]
fn creates_csv_file_when_passed_output_format() -> Result<()> {
    let output_file = NamedTempFile::new()?;
    let expected_text = "list,name,hex\nWikipedia,Pumpkin,#ff7518\n";

    Command::cargo_bin(pkg_name!())
        .unwrap()
        .arg("pumpkin")
        .arg("--output")
        .arg("csv")
        .arg("--file-path")
        .arg(output_file.path())
        .arg("--quiet")
        .assert()
        .success()
        .stdout("Creating csv file...\n");

    let text = std::fs::read_to_string(output_file.path())?;
    assert_eq!(text, expected_text);

    Ok(())
}
