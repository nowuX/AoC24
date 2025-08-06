use std::fs;

use anyhow::Result;

pub fn load_input(day_dir: &str) -> Result<String> {
    let data = fs::read_to_string(format!("{day_dir}/assets/input"))?;
    Ok(data)
}
