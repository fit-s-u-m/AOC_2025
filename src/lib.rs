use std::fs;
use std::str::FromStr;
use anyhow::{Result, Context};

pub fn read_one_per_line<T>(path:&str) -> Result<Vec<T>>
where
    T:FromStr,
    T::Err: std::fmt::Display,
{
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read the file {path}"))?;

    let mut result = Vec::new();

    for (i, line) in contents.lines().enumerate(){
        let value = line
        .parse::<T>()
        .map_err(|e| anyhow::anyhow!("Failed to parse line {}: `{}`: {}", i+1, line, e))?;
        result.push(value);
    }
    Ok(result)
}
