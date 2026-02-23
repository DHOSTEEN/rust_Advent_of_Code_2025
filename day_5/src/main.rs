
use day_5::parse_range;
use day_5::merge_ranges;
use day_5::error::Day5Error;


fn main() -> Result<(), Day5Error> {
    
    let file = file_reader::read_lines("day5_input.txt")?;
    let mut fresh_ids = Vec::new();
    let mut available_ids = Vec::new();
    let mut switch = true;
    for line in file.map_while(Result::ok) {

        if line.is_empty() {
            switch = !switch;
           
        } else if switch { 
            let new_ids = parse_range(&line)?;
            fresh_ids.push(new_ids);
        } else {
            available_ids.push(line.parse::<u64>()?);
        }
    }
    let mut count = 0;
    fresh_ids.sort_by_key(|arr| arr[0]);
    for available in available_ids {
        if fresh_ids.iter().any(|arr| arr[0] < available && arr[1] >= available){
            count += 1;
        }
    }
    println!("\ntask one result: {count}");


    let fresh_ids = merge_ranges(fresh_ids);

    let final_count:u64 =
    fresh_ids.into_iter()
        .map(|[start, end]| start.abs_diff(end) +1)
        .sum();

    println!("task two result: {final_count}");
    
    Ok(())
}
