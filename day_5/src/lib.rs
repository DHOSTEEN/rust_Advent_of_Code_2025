pub mod error;
use crate::error::Day5Error;

const START:usize = 0;
const END:usize = 1;

pub fn parse_range(raw_range: &str) -> Result<[u64;2], Day5Error> {
    let mut arr = [0u64;2];
    let split = raw_range.split_once("-").ok_or(Day5Error::RangeSplit)?;
    let (start, end) = split;
    arr[START] = start.parse::<u64>()?;
    arr[END] = end.parse::<u64>()?;
    Ok(arr)
}

///this is SUPER fast
pub fn merge_ranges(mut ranges: Vec<[u64; 2]>) -> Vec<[u64; 2]> {
    ranges.sort_unstable_by_key(|range| range[START]);// O(n log n)
    let mut merged:Vec<[u64;2]> = Vec::new();

    for range in ranges {//O(n)
        if let Some(last) = merged.last_mut() 
            && range[0] <= last[END] + 1 {//inclusive
                last[1] = last[END].max(range[END]);
                continue;
        }
        
        merged.push(range);
    }

    merged
}

///kept for comparison to O(n log n) function megre_ranges
///O(n^2)
#[allow(dead_code)]
pub fn combine_id_ranges(left :&[u64;2], right: &[u64;2]) -> Option<[u64;2]> {
    
    if left[START] < right[START] && left[END] > right[END] {
        ////println!("left: {},{}\nright: {},{}", left[START], left[END], right[START], right[END] );
        return Some(*left);
    }
    if right[START] < left[START] && right[END] > left[END] {
        return Some(*right);
    }

    //left bounds

    if left[START] < right[START]  && left[END] >= right[START]{
        //left is lower bounded
        //println!("LEFT HAS LOWER FIRST:\nleft: {},{}\nright: {},{}", left[START], left[END], right[START], right[END]);
        return Some([left[START], right[END]]);
    }
    if left[END] > right[END] {
        //left is higher bounded
        //println!("LEFT HAS HIGHER SECOND:\nleft: {},{}\nright: {},{}", left[START], left[END], right[START], right[END]);
        return Some([right[START], left[END]]);
    }

    //right bounds

    if right[START] < left[START] {
        return Some([right[START], left[END]]);
    }
    if right[END] > left[END] && left[END] >= right[START] {
        //println!("RIGHT HAS HIGHER SECOND:\nleft: {},{}\nright: {},{}", left[START], left[END], right[START], right[END]);
        return Some([left[START], right[END]]);
    }

    None
}