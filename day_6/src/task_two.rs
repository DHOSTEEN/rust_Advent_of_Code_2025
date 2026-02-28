use crate::task_one;
use day_6::Colum;
use day_6::Day6Error;

pub fn line_parseing(
    line: &str,
    all_cols: &mut Vec<Colum>
) -> Result<(), Day6Error> {
 
  
    for (i, bytes) in line.chars().enumerate() {
       
        let nth = all_cols.get_mut(i);
        if let Some(col) = nth {
    
            let raw_num = bytes.to_string();
            if raw_num == "*" || raw_num == "+" {
                col.add_symbol(&raw_num)?;
            } else{
            //col.add_num(&raw_num)?;
                col.add_string(&raw_num);
            }
        } else {
            let mut col = Colum::default();
            let raw_num = bytes.to_string();
            //col.add_num(&raw_num)?;
            col.add_string(&raw_num);
            all_cols.push(col);
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
