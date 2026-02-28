use day_6::Colum;
use day_6::Day6Error;
pub fn line_parseing(line: &str, all_cols: &mut Vec<Colum>) -> Result<(), Day6Error> {
    if line.starts_with("+") || line.starts_with("*") {
        parse_symbols(line, all_cols)?;
    } else {
        for (i, num) in line.split_whitespace().enumerate() {
            let nth = all_cols.get_mut(i);
            if let Some(col) = nth {
                col.add_num(num)?;
                //col.add_string(raw_num);
            } else {
                let mut col = Colum::default();

                col.add_num(num)?;
                //col.add_string(raw_num);
                all_cols.push(col);
            }
        }
    }
    Ok(())
}

pub fn parse_symbols(line: &str, all_cols: &mut [Colum]) -> Result<(), Day6Error> {
    for (i, symbol) in line.split_ascii_whitespace().enumerate() {
        let nth = all_cols.get_mut(i);
        if let Some(col) = nth {
            col.add_symbol(symbol)?;
        } //no need for else - always should have full colums at this stage
    }
    Ok(())
}
