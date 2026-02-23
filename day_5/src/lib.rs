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

///O(n log n) Sorts ranges by internal array's first element, merges ranges, and returns a new vector with merged ranges
///
///the returning vector contains:
///
///A) the unaltered elements of incoming ranges if no merge happens
///
///B) merged ranges done via the upper bound.
///
///This is done by first sorting the incoming list by the first element (START) to ensure no possible range overlaps are missed
///through a single pass merge.
///We then create a vector to store all results of the merge and loop through all incoming ranges.
///
///A merge occurs when the incoming range is both within the 'last' element of the merged list and has a greater upper bound
///with: 'if let Some(last) = merged.last() && range[START] < last[END]'
///
///ie: last = [10, 14] range = [12, 18]
///
///12 < 14
///
///Then we take the greater upper bound of both and update last:
///
///last = [10, 18]
///
///this allows all ranges to be merged in one pass of the incoming ranges, 0(n), so the greatest cost of this function is the sort
///# Example
///```
///use day_5::merge_ranges;
///let test_ranges = vec![[16,20], [3,5], [10,14], [12,18]];
///let expected = vec![[3,5], [10, 20]];
///let result = merge_ranges(test_ranges);
///assert_eq!(result, expected);
///```
pub fn merge_ranges(mut ranges: Vec<[u64; 2]>) -> Vec<[u64; 2]> {
    let test_ranges = vec![[16,20], [3,5], [10,14], [12,18]];
    ranges.sort_unstable_by_key(|range| range[START]);// O(n log n)
    //no need for START == START comparisons to care about orginal order so unstable is fine
    let mut merged:Vec<[u64;2]> = Vec::new();

    for range in ranges {//O(n)

        if let Some(last) = merged.last_mut() 
            && range[START] < last[END] {
                
                last[END] = last[END].max(range[END]);
                continue;
        }
        
        merged.push(range);
    }

    merged
}
