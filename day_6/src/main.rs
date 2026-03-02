use anyhow::Result;
use day_6::{Colum, Day6Error};
mod task_one;
mod task_two;

fn main() -> Result<()> {
    let file = file_reader::read_lines("day6_input.txt")?;

    let mut all_cols_task_one: Vec<Colum> = Vec::new();
    let mut all_rows_task_two: Vec<Vec<char>> = Vec::new();

    for line in file.map_while(Result::ok) {
        //println!("{line}");
        task_one::line_parseing(&line, &mut all_cols_task_one)?;

        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        all_rows_task_two.push(row);
    }
    let mut all_cols = task_two::line_parseing(&all_rows_task_two)?;

    // for all in all_cols.iter() {
    //     println!("{:?}", all);
    // }

    let result: u64 = all_cols_task_one.iter().map(|col| col.caculate()).sum();
    println!("{result:?}");

    let result: u64 = all_cols
        .iter_mut()
        .map(|col| col.caculate_t2())
        .sum::<Result<u64, Day6Error>>()?;
    println!("{result:?}");

    Ok(())
}
