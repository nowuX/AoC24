use std::{fmt::Debug, fs, str::FromStr};

use anyhow::Result;

pub fn load_input(day_dir: &str) -> Result<String> {
    let data = fs::read_to_string(format!("{day_dir}/assets/input"))?;
    Ok(data)
}

/// Parses multi-line input where each line contains two values separated by a delimiter.
pub fn parse_split_once<T>(data: &str, delimeter: &str) -> impl Iterator<Item = (T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.lines().map(move |line| {
        let (x, y) = line.split_once(delimeter).unwrap();
        (x.parse::<T>().unwrap(), y.parse::<T>().unwrap())
    })
}

/// Parses a string by splitting it on a delimiter and converting each part to type `T` and collects into a `Vec<T>`.
pub fn parse_split<T>(data: &str, delimeter: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.split(delimeter).map(|c| c.parse::<T>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::{parse_split, parse_split_once};

    #[test]
    fn test_parse_split_once_integers() {
        let input = "10   20\n30   40";
        let result: Vec<(i32, i32)> = parse_split_once(input, "   ").collect();
        assert_eq!(result, vec![(10, 20), (30, 40)]);
    }

    #[test]
    fn test_parse_split_once_strings() {
        let input = "hello,world\nfoo,bar";
        let result: Vec<(String, String)> = parse_split_once(input, ",").collect();
        assert_eq!(
            result,
            vec![
                ("hello".to_string(), "world".to_string()),
                ("foo".to_string(), "bar".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_split_integers() {
        let input = "1,2,3,4,5";
        let result: Vec<i32> = parse_split(input, ",").collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_split_floats() {
        let input = "1.5 2.7 9.1";
        let result: Vec<f64> = parse_split(input, " ").collect();
        assert_eq!(result, vec![1.5, 2.7, 9.1]);
    }

    #[test]
    #[should_panic]
    fn test_parse_split_once_missing_delimiter() {
        let input = "10 20"; // Missing the "   " delimiter
        let _: Vec<(i32, i32)> = parse_split_once(input, "   ").collect();
    }

    #[test]
    #[should_panic]
    fn test_parse_split_invalid_number() {
        let input = "1,abc,3";
        let _: Vec<i32> = parse_split(input, ",").collect();
    }
}
