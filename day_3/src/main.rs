use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod error;
use error::Day3Error;

fn main() -> Result<(),Day3Error> {

    let lines = read_lines("day3_input.txt")?;
    let mut voltage = vec![];
    for line in lines.map_while(Result::ok) {
        voltage.push(find_two_highest(&line)?);
    }
    let mut sum:u32 = 0;
    for battery in voltage.iter() {
        sum += battery;
    }
    println!("{sum}");
    Ok(())
}
/// from Rust By Example - read_lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
///in order to find the highest pair, we need to find the highest number in all of the line
///EXCEPT for the last pos - we are finding the A in AB
///we do this by getting a slice of all but the last character
///we keep track of the index so we need only check all characters AFTER the A
///we make a second slice from A position + 1, to the end of the slice
///finally we concatenate the chars together and parse them to create the numerical battery pair
fn find_two_highest(line: &str) -> Result<u32, Day3Error> {
   
    let slice = &line[0..line.len()-1];
    let mut highest:u32 = 0;
    let mut slice_start:usize = 0;
    let mut first_battery:char = 'a';
    let mut second_battery:char = 'a';
    for (index, ch) in slice.chars().enumerate() {
        let ch_int = ch.to_digit(10).ok_or(Day3Error::NotDigit)?;
        if ch_int > highest {
            highest = ch_int;
            first_battery = ch;//avoid repeated parse
            slice_start = index;
        }
    }
    let slice = &line[slice_start+1..];
    highest =0;

    for ch in slice.chars() {
        let ch_int = ch.to_digit(10).ok_or(Day3Error::NotDigit)?;
        if ch_int > highest {
            highest = ch_int;
            second_battery = ch;
        }
    }

    let result = format!("{first_battery}{second_battery}").parse::<u32>()?;
    Ok(result)
    
}