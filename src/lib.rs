use std::{fmt::Debug, fs, str::FromStr};

use anyhow::{Context, Result};

pub fn load_input(day: &str) -> Result<String> {
    let data =
        fs::read_to_string(format!("input/{day}.in")).context("⚠️\tCan not read the input file")?;
    Ok(data)
}

pub fn parse_split<T>(data: &str, delimiter: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.split(delimiter)
        .map(|word| word.parse::<T>().expect("⚠️\tInvalid parser type"))
}

pub fn parse_split_once<T>(data: &str, delimiter: &str) -> impl Iterator<Item = (T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.lines().map(move |line| {
        let (x, y) = line
            .split_once(delimiter)
            .expect("⚠️\tInvalid line delimeter");
        let parse = |s: &str| s.parse::<T>().expect("⚠️\tInvalid parser type");
        (parse(x), parse(y))
    })
}

pub fn parse_split_unsafe<T>(data: &str, delimiter: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.split(delimiter)
        .map(|word| unsafe { word.parse::<T>().unwrap_unchecked() })
}

pub fn parse_split_once_unsafe<T>(data: &str, delimiter: &str) -> impl Iterator<Item = (T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.lines().map(move |line| {
        let (x, y) = unsafe { line.split_once(delimiter).unwrap_unchecked() };
        let parse = |s: &str| unsafe { s.parse::<T>().unwrap_unchecked() };
        (parse(x), parse(y))
    })
}
