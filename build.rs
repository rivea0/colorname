use anyhow::Result;

fn main() -> Result<()> {
    let s = include_str!("./data/colorlists.json");
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("colorlists.json");
    std::fs::write(
        &dest_path,
        s
    )?;

    Ok(())
}