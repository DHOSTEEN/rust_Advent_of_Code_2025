use itertools::izip;
fn main() {
    let lines = file_reader::read_lines("day4_input.txt").unwrap();
    let mut raws = vec![];
    for line in lines.map_while(Result::ok) {
        //println!("{}", line);
        raws.push(line);
    }
    let grid = Grid::new(raws);

    let mut smart_rows = build_smart(&grid);
    count_valid(&smart_rows);

}

fn count_valid<'a>(rows: &[Row<'a>]) {
    let mut safe_count:u32 = 0;
    for r in rows {
        let middle:&[u8] = r.middle.unwrap().as_bytes();// will always have a middle due to logic before
        let above = r.above;
        let below = r.below;


        if let None = r.above {//then it MUST be below
            let my_zip = izip!(middle.windows(3), below.unwrap().as_bytes().windows(3)).enumerate();
            let len = my_zip.len();
            for (pos,(m,b)) in my_zip {
                //println!("{:?} -- {:?}", m, b);
                //explicity test first and last
                if pos == 0 && m[0] == 64 {
                    let mut count = count_row_slice(b, 0);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                    //test self
                }

                for i in 0..m.len() {
                    if m[i] == 64 && i == 1{
                        let mut count = 0;
                        //println!("the i pos is {i}");
                        //check below in i, i-1, i+1
                        count += count_row_slice(b, i);

                        count += count_neighbours(m);

                        if count < 4 {
                            safe_count += 1;
                        }
                    }
                    
                }//end of for loop

                if pos == len -1 && m[2] == 64 {
                    let mut count = count_row_slice(b, 2);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                }
                
            }
        } else if let None = r.below {// MUST have above
            let my_zip = izip!(middle.windows(3), above.unwrap().as_bytes().windows(3)).enumerate();
            let len = my_zip.len();
             for (pos, (m, a)) in my_zip {
                //println!("{:?} -- {:?}", m, a);

                if pos == 0 && m[0] == 64 {
                    let mut count = count_row_slice(a, 0);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                }
                for i in 0..m.len() {

                    if m[i] == 64 && i == 1 {
                        let mut count = count_row_slice(a, i);
                    
                        count += count_neighbours(m);

                        if count < 4 {
                            safe_count += 1;
                        }
                    }
                }
                if pos == len -1 && m[2] == 64 {
                    let mut count = count_row_slice(a, 2);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                }
            }
        }else {
            let my_zip = izip!(
                    middle.windows(3),
                    above.unwrap().as_bytes().windows(3),
                    below.unwrap().as_bytes().windows(3));
            let len = my_zip.len();
            for (pos,(m,a,b)) in my_zip.enumerate() {

                if pos == 0 && m[0] == 64{
                    let mut count = count_row_slice(a, 0);
                    count += count_row_slice(b, 0);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                }
                for i in 0..m.len() {

                    if i == 1 && m[i] == 64 {//huh can hard code...

                        let mut count = count_row_slice(a, i);
                        count += count_row_slice(b, i);
                        count += count_neighbours(m);
                        if count < 4 {
                            safe_count += 1;
                        }

                    }
                }//correct 
                
                if pos == len -1 && m[2] == 64 {
                    let mut count = count_row_slice(a, 2);
                    count += count_row_slice(b,2);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                }
                //println!("{:?} -- {:?} -- {:?}", a, m, b);
            }
        }

    }
    println!("row count: {safe_count}");
}

fn count_row_slice(row_slice: &[u8], i:usize) -> u32{
    let mut count = 0;
    let start = i.saturating_sub(1);
    let end = (i + 1).min(row_slice.len() - 1);
    for &ch in &row_slice[start..=end] {
        if ch == 64 {
            count += 1;
        }
    }
    count
}

fn count_row_end (row: &[u8], mut count:u32) -> (u32, u32) {
 
    let mut safe_count =0;

    if row[1] == 64 {//ie the middle A (B) A - row[1] is B, A are either ends of the slice
        count += 1;
    }
    if count < 4 {
        safe_count +=1;
    }
    (count, safe_count)
}

fn count_neighbours(row: &[u8]) -> u32 {
    let mut count = 0;
    if row[0] == 64 {
        count += 1;
    }
    if row[2] == 64 {
        count += 1;
    }
    count
}

fn build_smart<'a>(grid: &'a Grid) -> Vec<Row<'a>> {

    let mut smart_rows = vec![];
    let mut iter = grid.into_iter();
    let mut smart_row = Row::build(&iter.next().unwrap()[..]);
    smart_row.add_below(&iter.next().unwrap()[..]);
    smart_rows.push(smart_row);

    let mut buffer:[&str;2] = ["";2];
    let mut counter:usize = 0;
   
    for row in grid {
        //println!("{:?}", row.collect::<Vec<_>>());
        if counter > 1 {
            //middle
            smart_row = Row::build(buffer[1]);
            smart_row.add_above(buffer[0]);
            smart_row.add_below(&row[..]);

            buffer[0] = buffer[1];
            counter = 1;
            smart_rows.push(smart_row);
        }
        buffer[counter] = &row[..];
        counter += 1;
    }

    //need to add last row as middle, below None
    let mut rev_iter = grid.into_iter().rev();
    let mut smart_row = Row::build(&rev_iter.next().unwrap()[..]);
    smart_row.add_above(&rev_iter.next().unwrap()[..]);
    smart_rows.push(smart_row);
    smart_rows
    
}
#[derive(Debug)]
struct Grid{
    rows: Vec<String>,
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
impl Grid{
    pub fn new(rows: Vec<String>) -> Self {
        Grid {
            rows,
        }
    }
}
impl<'a> IntoIterator for &'a Grid{

    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }

}

#[cfg(test)]
mod test;