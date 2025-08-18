use std::{env, process::Command};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("Makima my beloved!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Use:\t\tcargo run <day>");
        println!("\t\tcargo run --bin day<XX>");
        return Ok(());
    }

    let day: u8 = args[1]
        .parse()
        .context(format!("Invalid day '{}'", args[1]))?;

    let bin_name = format!("day{day:02}");
    println!("🎄 Running Advent of Code - Day {day} 🎄");
    println!("{}", "=".repeat(35));

    let output = Command::new("cargo")
        .args(["run", "--bin", &bin_name])
        .output()
        .context("Error running day")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
