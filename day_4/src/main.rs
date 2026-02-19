use file_reader;
use itertools::Itertools;

fn main() {
    let lines = file_reader::read_lines("day4_test.txt").unwrap();
    let mut raws = vec![];
    for line in lines.map_while(Result::ok) {
        println!("{}", line);
        raws.push(line);
    }
    let mut grid = Grid::new(raws);
    /*for (i, x) in grid.into_iter().enumerate() {
        println!("{}: {}", i, x);
    }*/

    let mut iter = grid.rows.clone().into_iter();
    let slice = &iter.next().unwrap()[..];
    let mut first = Row::build(&slice);
    let slice = &iter.next().unwrap()[..];
    first.add_below(&slice);
    println!("below");
    for ch in first.below.unwrap().chars() {
        print!("{ch}");
    }
    grid.add_smart_row(first);
  
    let mut buffer:[&str;2] = ["";2];
    let mut counter:usize = 0;
    let mut smart_rows = vec![];
    for row in &grid {
        //println!("{:?}", row.collect::<Vec<_>>());
        if counter > 1 {
            //middle
            let mut smart_row = Row::build(buffer[1]);
            //above
            smart_row.add_above(buffer[0]);
            smart_row.add_below(&row[..]);

            buffer[0] = buffer[1];
            counter = 1;
            smart_rows.push(smart_row);
        }
        buffer[counter] = &row[..];
        counter += 1;
    }

    let mut rev_iter = grid.into_iter().rev();
    let mut smart_row = Row::build(&rev_iter.next().unwrap()[..]);
    smart_row.add_above(&rev_iter.next().unwrap()[..]);
    smart_rows.push(smart_row);
    for s in smart_rows {
        println!("{s:?}");
    }

}
#[derive(Debug)]
struct Grid<'a>{
    rows: Vec<String>,
    smart_rows: Vec<Row<'a>>,
}
#[derive(Debug, Copy, Clone)]
struct Row<'a>{

    pub above: Option<&'a str>,
    pub middle: Option<&'a str>,
    pub below: Option<&'a str>,
}
impl<'a> Row<'a> {
    pub fn new() -> Self {
        Row {

            above: None,
            middle: None,
            below: None
        }
    }
    pub fn build(current: &'a str) -> Self {
        Row {
            
            above: None,
            middle: Some(current),
            below: None,
        }
    }
    pub fn add_above(&mut self, slice: &'a str ) {
        self.above = Some(slice);
    }
    pub fn add_below(&mut self, slice: &'a str) {
        self.below = Some(slice);
    }
}
impl<'a> Grid<'a> {
    pub fn new(rows: Vec<String>) -> Self {
        Grid {
            rows,
            smart_rows: vec![],
        }
    }

    pub fn add_smart_row(&mut self, smart_row: Row<'a>) {
        self.smart_rows.push(smart_row);
    }
}
impl<'a> IntoIterator for &'a Grid<'a>{

    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }

}
