use std::result;

use anyhow::Result;
use day_6::{Colum, OperationSymbol};
mod task_one;
mod task_two;
fn main() -> Result<()> {
    let file = file_reader::read_lines("day6_input.txt")?;

    let mut all_cols_task_one: Vec<Colum> = Vec::new();
    let mut all_cols_task_two: Vec<Colum> = Vec::new();
    let mut col_width: usize = 0;

    for line in file.map_while(Result::ok) {

        println!("{line}");

        task_one::line_parseing(&line, &mut all_cols_task_one)?;
        task_two::line_parseing(&line, &mut all_cols_task_two)?;
    }

    // for all in all_cols {
    //     println!("{:?}", all);
    // }

    let result: u64 = all_cols_task_one.iter().map(|col| col.caculate()).sum();
    println!("{result:?}");

    let cols:Vec<_> = all_cols_task_two.iter().map(|col| col.caculate_t2()).collect();
    let mut symbol:OperationSymbol = OperationSymbol::Plus;
    let mut partial_result:u64 = 0;
    let mut result:u64 = 0;
    for (num, maybe_symbol) in cols.into_iter().map_while(Result::ok) {
        use OperationSymbol::*;
        println!("Number incoming: {:?}", num);
        if let Some(new_symbol) = maybe_symbol{        
            result += partial_result;
            println!("AFTER NEW COL: partial: {} full: {}", partial_result, result);
            symbol = new_symbol;
            partial_result = 0;
        }
        
        match symbol {
            Multiply => {if partial_result == 0 {
                partial_result += 1;
                }
                if let Some(num) = num{
                    partial_result *= num;
                }
            }
            Plus => if let Some(num) = num{
                    partial_result += num;
                }
        }
        println!("partial: {} full: {}", partial_result, result);
    } 
    println!("{result:?}");

    Ok(())
}
