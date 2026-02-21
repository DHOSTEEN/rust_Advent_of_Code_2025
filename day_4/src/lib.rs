use itertools::izip;

const ABOVE:usize = 0;
const MIDDLE:usize = 1;
const LEFT:usize = 0;
const RIGHT:usize = 2;
const TARGET:u8 = 64;

#[derive(Debug)]
pub struct Grid{
    pub rows: Vec<String>,
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

#[derive(Debug, Copy, Clone)]
pub struct Row<'a>{

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

pub fn build_smart<'a>(grid: &'a Grid) -> Vec<Row<'a>> {

    let mut smart_rows = vec![];
    let mut iter = grid.into_iter();
    let mut smart_row = Row::build(&iter.next().unwrap()[..]);
    smart_row.add_below(&iter.next().unwrap()[..]);
    smart_rows.push(smart_row);

    let mut buffer:[&str;2] = ["";2];
    let mut counter:usize = 0;
   
    for row in grid {
        if counter > 1 {
            smart_row = Row::build(buffer[MIDDLE]);
            smart_row.add_above(buffer[ABOVE]);
            smart_row.add_below(&row[..]);

            buffer[ABOVE] = buffer[MIDDLE];
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

pub fn count_valid<'a>(rows: &[Row<'a>]) -> (Vec<(usize, usize)>, u32) {
    let mut safe_count:u32 = 0;
    let mut col = 0;
    let mut row = 0;
    let mut to_remove = vec![];

    for r in rows {
        let middle:&[u8] = r.middle.unwrap().as_bytes();// will always have a middle due to logic before
        let above = r.above;
        let below = r.below;

        if let None = r.above {//then it MUST be below
            let my_zip = izip!(middle.windows(3), below.unwrap().as_bytes().windows(3)).enumerate();
            let len = my_zip.len();
            for (pos,(m,b)) in my_zip {
                //explicity test first and last
                if pos == 0 && m[LEFT] == TARGET {
                    let mut count = count_row_slice(b, LEFT);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;

                    if s > 0 {
                        row = pos;
                        to_remove.push((col, row));
                    }
                }
                if m[MIDDLE] == TARGET {
                    let mut count = 0;
                    //println!("the i pos is {i}");
                    //check below in i, i-1, i+1
                    count += count_row_slice(b, MIDDLE);

                    count += count_neighbours(m);

                    if count < 4 {
                        safe_count += 1;
                        row = pos + MIDDLE;
                        to_remove.push((col, row));
                    }
                }
                    
                if pos == len -1 && m[RIGHT] == TARGET {
                    let mut count = count_row_slice(b, RIGHT);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                    if s > 0 {
                        row = pos + RIGHT;
                        to_remove.push((col, row));
                    }
                }
                
            }
        } else if let None = r.below {// MUST have above
            let my_zip = izip!(middle.windows(3), above.unwrap().as_bytes().windows(3)).enumerate();
            let len = my_zip.len();

            for (pos, (m, a)) in my_zip {
                //println!("{:?} -- {:?}", m, a);
                if pos == 0 && m[LEFT] == TARGET {
                    let mut count = count_row_slice(a, LEFT);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                    if s > 0 {
                        row = pos;
                        to_remove.push((col, row));
                    }
                }
                if m[MIDDLE] == TARGET {
                    let mut count = count_row_slice(a, MIDDLE);
                
                    count += count_neighbours(m);

                    if count < 4 {
                        safe_count += 1;

                        row = pos + MIDDLE;
                        to_remove.push((col, row));
    
                    }
                }
                if pos == len -1 && m[RIGHT] == TARGET {
                    let mut count = count_row_slice(a, RIGHT);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                    if s > 0 {
                        row = pos + RIGHT;
                        to_remove.push((col, row));
                    }
                }
            }
        } else {//must have both above and below rows
            let my_zip = izip!(
                    middle.windows(3),
                    above.unwrap().as_bytes().windows(3),
                    below.unwrap().as_bytes().windows(3));
            let len = my_zip.len();
            for (pos,(m,a,b)) in my_zip.enumerate() {

                if pos == 0 && m[LEFT] == TARGET{
                    let mut count = count_row_slice(a, LEFT);
                    count += count_row_slice(b, LEFT);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    safe_count += s;
                    if s > 0 {
                        row = pos;
                        to_remove.push((col, row));
                    }
                }
                if m[MIDDLE] == TARGET {//huh can hard code...

                    let mut count = count_row_slice(a, MIDDLE);
                    count += count_row_slice(b, MIDDLE);
                    count += count_neighbours(m);
                    if count < 4 {
                        safe_count += 1;
                        
                        row = pos + MIDDLE;
                        to_remove.push((col, row));
                        
                    }
                }       
                if pos == len -1 && m[RIGHT] == TARGET {

                    let mut count = count_row_slice(a, RIGHT);
                    count += count_row_slice(b, RIGHT);
                    let (c, s) = count_row_end(m, count);
                    count += c;
                    if s > 0 {
                        row = pos + RIGHT;
                        to_remove.push((col, row));
                    }
                    safe_count += s;
                }
            }
        }
        col += 1;

    }

    (to_remove, safe_count)
}
///this function needs to sanitise the start and end of the slice in order to deal with the start and end of a line
fn count_row_slice(row_slice: &[u8], i:usize) -> u32 {
    let mut count = 0;
    let start = i.saturating_sub(1);
    let end = (i + 1).min(row_slice.len() - 1);
    for &ch in &row_slice[start..=end] {
        if ch == TARGET {
            count += 1;
        }
    }
    count
}

fn count_row_end (row: &[u8], mut count:u32) -> (u32, u32) {
 
    let mut safe_count =0;

    if row[MIDDLE] == TARGET {
        count += 1;
    }
    if count < 4 {
        safe_count += 1;
    }
    (count, safe_count)
}

fn count_neighbours(row: &[u8]) -> u32 {
    let mut count = 0;
    if row[LEFT] == TARGET {
        count += 1;
    }
    if row[RIGHT] == TARGET {
        count += 1;
    }
    count
}

#[cfg(test)]
mod test;