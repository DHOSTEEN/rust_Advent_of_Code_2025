use anyhow::Result;
use day_6::Colum;
mod task_one;
mod task_two;
fn main() -> Result<()> {
    let file = file_reader::read_lines("day6_input.txt")?;

    let mut all_cols_task_one: Vec<Colum> = Vec::new();
    let mut all_cols_task_two: Vec<Colum> = Vec::new();
    let mut col_width: usize = 0;
    let mut first = true;
    for line in file.map_while(Result::ok) {
        if first {
            col_width = task_two::calculate_col_width(&line);
            println!("WIDTH: {}", col_width);
            first = !first;
        }
        println!("{line}");

        task_one::line_parseing(&line, &mut all_cols_task_one)?;
        task_two::line_parseing(&line, &mut all_cols_task_two, col_width)?;
    }

    // for all in all_cols {
    //     println!("{:?}", all);
    // }

    let result: u64 = all_cols_task_one.iter().map(|col| col.caculate()).sum();
    println!("{result:?}");

    let result: u64 = all_cols_task_two
        .iter_mut()
        .map(|col| col.caculate_t2())
        .sum();
    println!("{result:?}");

    Ok(())
}
