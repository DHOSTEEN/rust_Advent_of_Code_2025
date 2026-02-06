use std::fs;
use fancy_regex::Regex;

fn main() {

    let file_string = fs::read_to_string("day_two_test.txt").expect("no file");
    let string_split = Full_ID::new_IDs(&file_string);
    //let split = string_split.split(",");
    //println!("splits: {:?}", string_split);
    let mut count = 0;
    for i in string_split {


            if i.first.is_valid() {
                count += 1;
            }
            if i.last.is_valid() {
                count += 1;
            }
        
    }
    println!("{count}");
}
#[derive(Debug)]
struct Full_ID<'a> {
    first: ID<'a>,
    last: ID<'a>,
}

impl <'a>Full_ID<'a> {
    pub fn new_IDs (all_ids: &'a str) -> Vec<Self> {
        let mut all_ID:Vec<Self> = vec![];
        for raws in all_ids.split(",") {
            let raws = raws.trim();
            let mut raw_split = raws.split("-");
            let id = Full_ID {
                first: ID::new(raw_split.next()),
                last: ID::new(raw_split.next()),
            };
            all_ID.push(id);
        }
        all_ID
    }

}
#[derive(Debug)]
struct ID<'a> {
    id: &'a str,
}

impl<'a> ID<'a> {
    pub fn new (raw: Option<&'a str>) -> Self {
        ID {
            id: raw.unwrap(),
        }
    }

    fn is_valid(&self) -> bool {
        
        let rs = Regex::new(r"^(\d+)\1").unwrap();
        println!("{:?} is {}", self.id, Regex::is_match(&rs, self.id).unwrap());
        Regex::is_match(&rs, self.id).unwrap()
    }
}