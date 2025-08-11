use std::time::Instant;

use anyhow::Result;

macro_rules! run_days {
    ($($day:ident),*) => {
        $(
            {
                let input = utils::load_input(stringify!($day))?;
                let start = Instant::now();
                let part_1 = $day::part_1(&input)?;
                let duration = start.elapsed();
                println!("=> {}\n   Part 1: {:?}, {:?}", stringify!($day).to_uppercase(), part_1, duration);
                let start = Instant::now();
                let part_2 = $day::part_2(&input)?;
                let duration = start.elapsed();
                println!("   Part 2: {:?}, {:?}\n", part_2, duration)

            }
        )*
    };
}

fn main() -> Result<()> {
    println!("Makima my beloved!\n");
    run_days!(day01, day02, day03, day04, day05, day06, day07);
    Ok(())
}
