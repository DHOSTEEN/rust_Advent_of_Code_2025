use std::fs;
use fancy_regex::Regex;
use std::thread;
use std::sync::LazyLock;

use std::time::Instant;
mod error;
use error::Day2Error;

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

    let result:Result<u64, Day2Error> = thread::scope(|scope| {
    let chunked = full_id.range.chunks(chunk_size);
        
    let mut handles = vec![];
        for ids in chunked {
            
            handles.push(scope.spawn(move || -> Result<_, Day2Error> {
                let mut count = 0;
                for id in ids {
                    match day_switch {

                       DaySwitch::One => { 

                        let valid = id.is_valid()?;
                            if valid {
                                count += id.id.parse::<u64>()?;
                            }
                        },
                        DaySwitch::Two => {
                            let valid = id.is_valid_task_2()?;
                            if valid {
                                count += id.id.parse::<u64>()?;
                            }
                        }
                    }
                }
                Ok(count)
            }));
        }

        let mut result:u64 = 0;

        for handle in handles {
            let val = handle
                        .join()
                        .map_err(|_| Day2Error::ThreadPanic)??;
            result += val; 
        }
        Ok(result)

    });

    println!("{result:?}")
         
    
}

#[derive(Debug)]
struct FullId {
    range: Vec<ID>
}

impl FullId {
    pub fn new_ids(all_ids: &str) -> Result<Self, Day2Error> {
        let range = all_ids
            .split(',')
            .try_fold(Vec::new(), |mut range, raw| -> Result<_, Day2Error> {
                let mut parts = raw.trim().split('-');

                let start = parts
                    .next()
                    .ok_or(Day2Error::NoStart)?
                    .parse::<usize>()?;

                let end = parts
                    .next()
                    .ok_or(Day2Error::NoEnd)?
                    .parse::<usize>()?;

                for num in start..=end {
                    range.push(ID::new(num.to_string()));
                }

                Ok(range)
            })?;

        Ok(Self { range })
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

    fn is_valid(&self) -> Result<bool, Day2Error> {
        let result = DAY_ONE_REGEX.is_match(&self.id)?;
        Ok(result)
    }

    fn is_valid_task_2(&self) -> Result<bool, Day2Error> {
        let result = DAY_TWO_REGEX.is_match(&self.id)?;
        Ok(result)
    }
}