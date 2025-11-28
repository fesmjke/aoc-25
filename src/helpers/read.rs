use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a file line by line, returning an iterator over the lines.
pub fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
