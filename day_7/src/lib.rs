pub mod error;
pub use error::Day7Error;

#[derive(Debug, Default, Clone)]
pub struct TachyonBeam {
    pub current_point: Point,

    pub all_points: Vec<Point>,
    pub internal_count: u32,
}
#[derive(Debug, Default, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}
impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl TachyonBeam {
    pub fn new(current_point: Point) -> Self {
        Self {
            current_point,
            all_points: Vec::new(),
            internal_count: 0,
        }
    }
    pub fn fire(&mut self, grid: &mut Manifold) -> Result<Option<TachyonBeam>, Day7Error> {
        if grid.has_next(self.current_point.row) {
            let row = self.current_point.row;
            let col = self.current_point.col;
            match grid.get(self.current_point.row + 1, self.current_point.col) {
                '^' => {
                    self.all_points.push(Point::new(row, col));
                    self.current_point = Point::new(row + 1, col + 1);
                    self.internal_count += 1;
                    grid.grid[row][col] = '|';
                    Ok(Some(TachyonBeam::new(Point::new(row + 1, col - 1))))
                }
                '.' => {
                    self.all_points.push(Point::new(row, col));
                    self.current_point = Point::new(row + 1, col);
                    Ok(None)
                }
                _ => Err(Day7Error::InvalidPoint),
            }
        } else {
            Err(Day7Error::EOF)
        }
    }
}
#[derive(Debug, Default)]
pub struct Manifold {
    pub grid: Vec<Vec<char>>,
}

impl Manifold {
    pub fn add_row(&mut self, row: impl Iterator<Item = char>) {
        self.grid.push(row.collect());
    }
    pub fn get(&self, i: usize, j: usize) -> char {
        let j = j.min(self.grid[i].len());
        // println!("the j is: {} the len is: {}", j, self.grid[i].len());
        // println!("the i is: {} the len is: {}", i, self.grid.len());
        self.grid[i][j]
    }
    pub fn has_next(&self, pos: usize) -> bool {
        //println!("MY POS: {}, len{}", pos, self.grid.len());
        pos < self.grid.len() - 1
    }
}
