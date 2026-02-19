use std::fs::File;
use io::BufRead;
use std::path::Path;
use std::io;
/// from Rust By Example - read_lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

