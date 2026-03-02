use anyhow::Result;
use day_7::{Manifold, Point, TachyonBeam};

fn main() -> Result<()> {
    let file = file_reader::read_lines("test.txt")?;

    let mut tachyon_manifold_diagram = Manifold::default();
    for line in file.map_while(Result::ok) {
        //println!("{line}");
        tachyon_manifold_diagram.add_row(line.chars());
    }

    let mut rows = tachyon_manifold_diagram.grid.iter();
    let first = rows.next().unwrap();
    let mut beams: Vec<&TachyonBeam> = Vec::new();
    let mut start_point: TachyonBeam = TachyonBeam::default();
    for (i, start) in first.iter().enumerate() {
        if *start == 'S' {
            start_point = TachyonBeam::new(Point::new(0, i));
            beams.push(&start_point);
            break;
        }
    }

    let beams = recursive_fire(&mut tachyon_manifold_diagram, start_point);

    let mut count = 0;
    for beam in beams {
        // println!("{:?}", t);
        count += beam.internal_count;
    }
    // for row in tachyon_manifold_diagram.grid.iter() {
    //     println!("{:?}", row);
    // }
    println!("count: {}", count);
    Ok(())
}

fn recursive_fire(mainfold: &mut Manifold, mut beam: TachyonBeam) -> Vec<TachyonBeam> {
    match beam.fire(mainfold) {
        Err(_) => vec![beam],
        Ok(Some(new_beam)) => {
            let mut vec = Vec::new();
            vec.append(&mut recursive_fire(mainfold, beam));
            vec.append(&mut recursive_fire(mainfold, new_beam));
            vec
        }
        Ok(None) => recursive_fire(mainfold, beam),
    }
}
