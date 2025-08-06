use anyhow::Result;

macro_rules! run_days {
    ($($day:ident),*) => {
        $(
            {
                let input = utils::load_input(stringify!($day))?;
                println!("=> {}\n   Part 1: {:?}\n   Part 2: {:?}\n", stringify!($day).to_uppercase(), $day::part_1(&input)?, $day::part_2(&input)?);
            }
        )*
    };
}

fn main() -> Result<()> {
    println!("Makima my beloved!\n");

    run_days!(day01, day02, day03, day04);
    Ok(())
}
