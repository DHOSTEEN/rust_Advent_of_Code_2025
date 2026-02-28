use crate::task_one;
use day_6::Colum;
use day_6::Day6Error;

pub fn line_parseing(
    line: &str,
    all_cols: &mut Vec<Colum>,
    col_width: usize,
) -> Result<(), Day6Error> {
    if line.starts_with("+") || line.starts_with("*") {
        task_one::parse_symbols(line, all_cols)?;
    } else {
        for (i, bytes) in line.as_bytes().chunks(col_width + 1).enumerate() {
            let nth = all_cols.get_mut(i);
            if let Some(col) = nth {
                let raw_num = std::str::from_utf8(bytes)?;
                //col.add_num(&raw_num)?;
                col.add_string(raw_num);
            } else {
                let mut col = Colum::default();
                let raw_num = std::str::from_utf8(bytes)?;
                //col.add_num(&raw_num)?;
                col.add_string(raw_num);
                all_cols.push(col);
            }
        }
    }
    Ok(())
}

pub fn calculate_col_width(line: &str) -> usize {
    let mut max: u32 = 0;
    let mut count = 0;
    //println!("{line}");
    for ch in line.chars() {
        if ch == ' ' {
            if max < count {
                max = count;
            }
            count = 0;
        } else {
            count += 1;
        }
    }
    max as usize
}
