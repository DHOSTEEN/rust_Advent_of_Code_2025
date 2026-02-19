
 use super::*;

    #[test]
    fn middle_row_correct() {
        let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut test_lines:Vec<String> = vec![];
        for line in test_input.lines(){
            test_lines.push(line.to_string());
        }
        let test_grid = Grid::new(test_lines);
        let smart_rows = build_smart(&test_grid);
        for (row, expected) in smart_rows.into_iter().zip(test_grid.into_iter()) {
            assert_eq!(expected, row.middle.unwrap());
        }
    }