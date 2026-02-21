
use day_4::count_valid;
use day_4::Grid;
use day_4::build_smart;
mod error;
use error::Day4Error;

fn main() -> Result<(), Day4Error> {
    let lines = file_reader::read_lines("day4_input.txt")?;
    let mut raws = vec![];
    for line in lines.map_while(Result::ok) {
        //println!("{}", line);
        raws.push(line);
    }
    let mut grid = Grid::new(raws);
    let mut final_count = 0_u32;
    let mut first_count = 0_u32;

    loop {
        let smart_rows = build_smart(&grid);

        let removals_and_count = count_valid(&smart_rows.unwrap());
        let (removals, count) = removals_and_count;
        if count == 0 {break;}
        final_count += count;
        if first_count == 0 {
            first_count = count;
        }

        for (col, row) in removals.iter() {

            let stuff = grid.rows.get_mut(*col).unwrap();
            stuff.replace_range(*row..=*row, "x");
        }
        //for (i,x) in grid.rows.iter_mut().enumerate() {
        //    println!("{:?}",x );
        //}
    }
    println!("first count: {}\nfinal count: {}", first_count, final_count);
    Ok(())
}
