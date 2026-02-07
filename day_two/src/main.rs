use std::fs;
use fancy_regex::Regex;
use std::thread;
use std::sync::LazyLock;
use std::num::ParseIntError;

use std::time::Instant;

static MY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)\1$").unwrap());

fn main() {

    let file_string = fs::read_to_string("day_two_input.txt").expect("no file");
    let string_split = Full_ID::new_ids(&file_string);

    let number_of_threads = std::thread::available_parallelism().unwrap().get();
    let target_chunks = number_of_threads;//4-8 heuristic for  - CORE dependant heuristic is approx num of cores
    let chunk_size = string_split.range.len() / target_chunks;

    let start = Instant::now();
    
    let result:u64 = thread::scope(|scope| {
        let chunked = string_split.range.chunks(chunk_size);
        
        let mut handles = vec![];
        for ids in chunked {
            
            handles.push(scope.spawn(move || {
                let mut count = 0;
                for id in ids {
                    if let Ok(true) = id.is_valid() {

                        count += id.id.parse::<u64>().unwrap();
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
    let duration = start.elapsed();
    println!("time: {:?}", duration.as_millis());
}

#[derive(Debug)]
struct Full_ID {
    range: Vec<ID>
}

impl Full_ID{
    pub fn new_ids<'a> (all_ids: &'a str) -> Self {

        let range = all_ids
        .split(',')
        .flat_map(|raw| {
            let mut parts = raw.trim().split('-');

            let start = parts.next().unwrap().parse::<usize>()?;
            let end   = parts.next().unwrap().parse::<usize>()?;

            Ok::<Vec<ID>, ParseIntError>((start..=end).map(|i| ID::new(i.to_string())).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();

        Full_ID{
            range,
        }
       
    }

}
#[derive(Debug)]
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
        MY_REGEX.is_match(&self.id)
    }
}