use anyhow::Result;
use day_6::Colum;
fn main() -> Result<()> {
    let file = file_reader::read_lines("day6_input.txt")?;

    let mut all_cols: Vec<Colum> = Vec::new();
    for line in file.flatten() {
        //println!("{line}");
        line_parseing(line, &mut all_cols)?;
    }

    let result: u64 = all_cols.iter().map(|col| col.caculate()).sum();
    println!("{result:?}");
    Ok(())
}

fn line_parseing(line: String, all_cols: &mut Vec<Colum>) -> Result<()> {
    if line.starts_with("+") || line.starts_with("*") {
        for (i, symbol) in line.split_ascii_whitespace().enumerate() {
            let nth = all_cols.get_mut(i);
            if let Some(col) = nth {
                col.add_symbol(symbol)?;
            } //no need for else - always should have full colums at this stage
        }
    } else {
        for (i, num) in line.split_whitespace().enumerate() {
            let nth = all_cols.get_mut(i);
            if let Some(col) = nth {
                col.add_num(num)?;
            } else {
                let mut col = Colum::default();
                col.add_num(num)?;
                all_cols.push(col);
            }
        }
    }

    Ok(())
}
