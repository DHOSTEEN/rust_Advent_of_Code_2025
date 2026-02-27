use day_6::Day6Error;
use day_6::Colum;
pub fn line_parseing(line: &String, all_cols: &mut Vec<Colum>) -> Result<(), Day6Error> {
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