use day_6::Colum;
use day_6::Day6Error;

pub fn line_parseing(rows: &[Vec<char>]) -> Result<Vec<Colum>, Day6Error> {
    let row_count = rows.len();
    let col_count = rows[0].len();

    let mut cols = vec![Vec::with_capacity(row_count); col_count];

    for row in rows {
        for (index, elem) in row.iter().enumerate() {
            cols[index].push(*elem);
        }
    }
    let mut all_cols: Vec<Colum> = Vec::new();
    let mut fat_col = Colum::default();
    'outer: for c in cols.iter().rev() {
        let mut concat_num = String::new();
        for elem in c.iter() {
            if *elem == '+' || *elem == '*' {
                fat_col.add_symbol(&elem.to_string())?;
                fat_col.add_string(concat_num);
                all_cols.push(fat_col);

                fat_col = Colum::default();
                continue 'outer;
            } else if *elem != ' ' {
                concat_num.push(*elem);
            }
        }
        if !concat_num.is_empty() {
            fat_col.add_string(concat_num);
        }
    }
    Ok(all_cols)
}
