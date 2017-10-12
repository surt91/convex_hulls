use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use itertools::Itertools;

pub fn svg(pointset: &[f64], hull: &[f64], filename: &str) -> Result<(), io::Error> {
    let path = Path::new(filename);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;

    write!(file, "<?xml version='1.0' encoding='UTF-8'?> \n\
                <!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>\n\
                <svg xmlns='http://www.w3.org/2000/svg'\n\
                xmlns:xlink='http://www.w3.org/1999/xlink' xmlns:ev='http://www.w3.org/2001/xml-events'\n\
                version='1.1' baseProfile='full' width='800px' height='800px' viewBox='-0.05 -0.05 1.10 1.10'>\n")?;
    for i in pointset.iter().tuples::<(_, _)>() {
        write!(file, "<circle cx='{}' cy='{}' r='0.01' stroke='black' stroke-width='0' />\n", i.0, i.1)?;
    }

    for (a, b) in hull.iter()
                      .tuples::<(_, _)>()
                      .tuple_windows::<(_, _)>()
    {
        write!(file, "<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='red' stroke-width='0.002' />\n", a.0, b.0, a.1, b.1)?;
    }
    write!(file, "</svg>\n")?;
    Ok(())
}
