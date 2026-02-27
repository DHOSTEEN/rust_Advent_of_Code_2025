
pub fn calculate_col_width(line: &String) -> usize {
    let mut max:u32 = 0;
    let mut count = 0;
    println!("{line}");
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
