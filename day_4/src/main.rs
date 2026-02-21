
use day_4::count_valid;
use day_4::Grid;
use day_4::build_smart;

fn main() {
    let lines = file_reader::read_lines("day4_input.txt").unwrap();
    let mut raws = vec![];
    for line in lines.map_while(Result::ok) {
        //println!("{}", line);
        raws.push(line);
    }
    let mut grid = Grid::new(raws);
    let mut final_count = 0_u32;
    let mut first_count = 0_u32;

    loop {
        let mut smart_rows = build_smart(&grid);
        let removals_and_count = count_valid(&smart_rows);
        let (removals, count) = removals_and_count;
        if count == 0 {break;}
        final_count += count;
        if first_count == 0 {
            first_count = count;
        }

        //println!("\nAAAAAAAAAAAA\n");
        for (col, row) in removals.iter() {
      
           // println!("col, row: {},{}", col, row);
            let mut stuff = grid.rows.get_mut(*col).unwrap();
            stuff.replace_range(*row..=*row, "x");
        
        //x.replace_range(1..2, "7");
        }
        //for (i,x) in grid.rows.iter_mut().enumerate() {
        //    println!("{:?}",x );
        //}
    }
    println!("first count: {}\nfinal count: {}", first_count, final_count);

}


#[cfg(test)]
mod test;