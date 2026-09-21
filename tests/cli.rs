use assert_cmd::{Command, pkg_name};
use predicates::prelude::*;

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
