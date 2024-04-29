use std::fs;

fn main() -> anyhow::Result<()> {
    let code = iter_n_gen::gen_all()?.to_string();
    fs::write("src/generate.rs", code)?;
    Ok(())
}
