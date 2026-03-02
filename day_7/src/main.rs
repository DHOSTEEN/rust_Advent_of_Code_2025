use anyhow::Result;
use day_7::Manifold;

fn main() -> Result<()> {
    let file = file_reader::read_lines("test.txt")?;

    let mut tachyon_manifold_diagram = Manifold::default();
    for line in file.map_while(Result::ok) {
        println!("{line}");
        tachyon_manifold_diagram.add_row(line.chars());
    }

    for rows in tachyon_manifold_diagram.grid.iter() {
        println!("{:?}", rows);
    }
    Ok(())
}
