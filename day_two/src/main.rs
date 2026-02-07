use std::fs;
use fancy_regex::Regex;
use std::thread;
use std::sync::LazyLock;

static MY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)\1$").unwrap());



fn main() {

    let file_string = fs::read_to_string("day_two_input.txt").expect("no file");
    let string_split = Full_ID::new_IDs(&file_string);

    let number_of_threads = std::thread::available_parallelism().unwrap().get();
    println!("MINE THREADS: {number_of_threads:?}");
    let target_chunks = number_of_threads;//4-8 heuristic
    let chunk_size = string_split.range.len() / target_chunks;

    println!("range of list: {:?}\nchunk size: {:?}", string_split.range.len(), chunk_size);

    
    let result:u64 = thread::scope(|scope| {
        let chunked = string_split.range.chunks(chunk_size);
        
        let mut handles = vec![];
        for ids in chunked {
            
            handles.push(scope.spawn(move || {
                let mut count = 0;
                for id in ids {
                    if id.is_valid() {
                        println!("{}", id.id);
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
}
    //
 

#[derive(Debug)]
struct Full_ID {
    range: Vec<ID>
}

impl Full_ID{
    pub fn new_IDs<'a> (all_ids: &'a str) -> Self {
        let mut all_ID:Vec<ID> = vec![];
        for raws in all_ids.split(",") {
            let raws = raws.trim();
            let mut raw_split = raws.split("-");
            let start = raw_split.next().unwrap().parse::<usize>().unwrap();
            let end = raw_split.next().unwrap().parse::<usize>().unwrap() + 1;

            for i in start..end {

                all_ID.push(ID::new(i.to_string()));
            }
            
        }
        Full_ID{
            range: all_ID
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

    fn is_valid(&self) -> bool {
        

        //println!("{:?} is {}", &self.id.to_string(), Regex::is_match(&rs, &self.id.to_string()).unwrap());
        //Regex::is_match(&rs, &self.id.to_string()).unwrap()
        /*if let Some(stuff) = MY_REGEX.find(&self.id.to_string()).unwrap(){
            let first = &stuff;
            println!("first val: {:?}", first);
            //return self.id.to_string().len() == self.length;
            return true;
        }
        
        false*/
        match MY_REGEX.find(&self.id).unwrap() {
            Some(_) => return true,
            None => return false,
        }

    }
}