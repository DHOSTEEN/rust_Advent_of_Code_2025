use anyhow::Result;
use day_6::Colum;
mod task_one;
mod task_two;
fn main() -> Result<()> {
    let file = file_reader::read_lines("day6_test.txt")?;

    let mut all_cols_task_one: Vec<Colum> = Vec::new();

    let mut first = true;
    for line in file.map_while(Result::ok) {

        println!("{line}");

        task_one::line_parseing(&line, &mut all_cols_task_one)?;
        //task_two::line_parseing(&line, &mut all_cols_task_two, width)?;
        
    }
 
    // for all in all_cols {
    //     println!("{:?}", all);
    // }

    let result: u64 = all_cols_task_one
        .iter().map(|col| col.caculate()).sum();
    println!("{result:?}");

    let result: u64 = all_cols_task_one.iter_mut().map(|col| col.caculate_t2()).sum();
    println!("{result:?}");
 
    Ok(())
}


