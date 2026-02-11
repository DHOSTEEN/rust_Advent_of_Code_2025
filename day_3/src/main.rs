use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod error;
use error::Day3Error;

const TASK1_BATTERY_NUM:usize = 2;
const TASK2_BATTERY_NUM:usize = 12;

fn main() -> Result<(),Day3Error> {

    let lines = read_lines("day3_input.txt")?;
    let mut voltage_task1 = vec![];
    let mut voltage_task2 = vec![];
    for line in lines.map_while(Result::ok) {
        voltage_task1.push(find_highest_joltage(&line, TASK1_BATTERY_NUM)?);
        voltage_task2.push(find_highest_joltage(&line, TASK2_BATTERY_NUM)?);
    }
    let mut sum:u64 = 0;
    for battery in voltage_task1.iter() {
        sum += battery;
    }
    println!("{sum}");
    sum = 0;
     for battery in voltage_task2.iter() {
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

///In order to find valid battery combinations, we use &str slices, with the end point being the length of the
///string - number of batteries. We keep track of the start and use this as a moving slider. This means that we
///only need to check the highest digit in each slice (O(n)) in order to get the next valid battery
///we concatenate each char found to a string and then parse to u64
fn find_highest_joltage(line: &str, num_batteries: usize) -> Result<u64, Day3Error> {

    let mut battery_bank = String::new();
    let mut start = 0;
    for end in (0..num_batteries).rev() {
        let slice = &line[start..line.len()-end];
        let (index, battery) = find_highest_in_slice(slice)?;
        start += index;
        battery_bank = format!("{battery_bank}{}", battery);
    }
    let result = battery_bank.parse::<u64>()?;
    Ok(result)
}

fn find_highest_in_slice(slice: &str) -> Result<(usize, char), Day3Error> {

    let mut highest:char = '0';
    let mut index:usize = 0;
    for (i, ch) in slice.chars().enumerate() {
        if !ch.is_digit(10){return Err(Day3Error::NotDigit);}
        if ch > highest {
            highest = ch;
            index = i;
        }
    }
    Ok((index+1, highest))
}