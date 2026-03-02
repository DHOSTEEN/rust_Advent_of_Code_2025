#[derive(Debug, Default)]
struct TachyonBeam {
    current_point: TachyonPoint,
    all_points: Vec<TachyonPoint>,
}
#[derive(Debug, Default)]
struct TachyonPoint {
    point: usize,
}

impl TachyonBeam {}
#[derive(Debug, Default)]
pub struct Manifold {
    pub grid: Vec<Vec<char>>,
}

impl Manifold {
    pub fn add_row(&mut self, row: impl Iterator<Item = char>) {
        self.grid.push(row.collect());
    }
}
