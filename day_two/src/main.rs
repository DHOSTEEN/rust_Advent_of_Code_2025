use std::fs;
use fancy_regex::Regex;
use std::thread;
use std::sync::LazyLock;
use std::num::ParseIntError;
use std::error::Error;


use std::time::Instant;

static DAY_ONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)\1$").unwrap());
static DAY_TWO_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)\1+$").unwrap());

fn main() {

    let file_string = fs::read_to_string("day_two_input.txt").expect("no file");
    let string_split = FullId::new_ids(&file_string).unwrap();
    let number_of_threads = std::thread::available_parallelism().unwrap().get();
    let target_chunks = number_of_threads;//4-8 heuristic for  - CORE dependant heuristic is approx num of cores
    let chunk_size = string_split.range.len() / target_chunks;

    let start = Instant::now();   
    run(&string_split, chunk_size, DaySwitch::One);    
    let duration = start.elapsed();
    println!("time task one: {:?}", duration.as_millis());

    let start = Instant::now();   
    run(&string_split, chunk_size, DaySwitch::Two);    
    let duration = start.elapsed();
    println!("time task two: {:?}", duration.as_millis());

}

#[derive(Debug, Clone, Copy)]
enum DaySwitch {
    One,
    Two,
}

fn run(full_id: &FullId, chunk_size: usize, day_switch: DaySwitch) 
{

    let result:u64 = thread::scope(|scope| {
    let chunked = full_id.range.chunks(chunk_size);
        
    let mut handles = vec![];
        for ids in chunked {
            
            handles.push(scope.spawn(move || {
                let mut count = 0;
                for id in ids {
                    match day_switch {

                       DaySwitch::One => { 
                            if let Ok(true) = id.is_valid() {

                                count += id.id.parse::<u64>().unwrap();
                            }
                        },
                        DaySwitch::Two => {
                            if let Ok(true) = id.is_valid_task_2() {
                                count += id.id.parse::<u64>().unwrap();
                            }
                        }
                    }
                }
                count
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum()
    });
         
    println!("{result}");
}

#[derive(Debug)]
struct FullId {
    range: Vec<ID>
}

impl FullId{
    pub fn new_ids<'a> (all_ids: &'a str) -> Result<Self, ParseIntError> {

        let range = all_ids
        .split(',')
        .try_fold(Vec::new(),|mut range, raw| {
            let mut parts = raw.trim().split('-');

            let start = parts.next().unwrap().parse::<usize>()?;
            let end   = parts.next().unwrap().parse::<usize>()?;

            for num in start..=end {
                range.push(ID::new(num.to_string()));
            }
            Ok(range)
        })?;
        
        Ok(FullId{
            range,
        })
    }
}
#[derive(Debug, Clone)]
struct ID {
    id: String,
}

impl ID {
    pub fn new (id: String) -> Self {
        ID {
            id,
        }
    }

    fn is_valid(&self) -> Result<bool, fancy_regex::Error> {
        DAY_ONE_REGEX.is_match(&self.id)
    }

    fn is_valid_task_2(&self) -> Result<bool, fancy_regex::Error> {
        DAY_TWO_REGEX.is_match(&self.id)
    }
}