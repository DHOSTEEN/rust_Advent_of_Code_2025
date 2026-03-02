use day_6::Colum;
use day_6::Day6Error;

pub fn line_parseing<'a>(
    rows: &[Vec<char>]
) -> Result<Vec<Vec<char>>, Day6Error> {
    let row_count = rows.len();
    let col_count = rows[0].len();

    let mut cols = vec![Vec::with_capacity(row_count); col_count];

    for row in rows {
        for (j, elem) in row.into_iter().enumerate() {
            cols[j].push(*elem);
        }
    }
    for x in cols.iter() {
        println!("{:?}", x);
    }
    Ok(cols)
}
