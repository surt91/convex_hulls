use std::io;
use std::fs::File;
use std::path::Path;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use itertools::Itertools;

pub struct SVG {
    buffer: String,
}

impl SVG {
    pub fn new() -> SVG {
        let buffer = "<?xml version='1.0' encoding='UTF-8'?> \n\
                    <!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>\n\
                    <svg xmlns='http://www.w3.org/2000/svg'\n\
                    xmlns:xlink='http://www.w3.org/1999/xlink' xmlns:ev='http://www.w3.org/2001/xml-events'\n\
                    version='1.1' baseProfile='full' width='800px' height='800px' viewBox='-0.05 -0.05 1.10 1.10'>\n
                    <rect x='-0.05' y='-0.05' width='1.10' height='1.10' fill='white' />\n".to_string();

        SVG {
            buffer
        }
    }

    pub fn save(&mut self, filename: &str) -> Result<(), io::Error> {
        let path = Path::new(filename);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = File::create(&path)?;

        writeln!(self.buffer, "</svg>").expect("write error");

        write!(file, "{}", &self.buffer)?;
        Ok(())
    }

    pub fn points(&mut self, pointset: &[f64], color: &str) {
        for i in pointset.iter().tuples::<(_, _)>() {
            writeln!(self.buffer, "<circle cx='{}' cy='{}' r='0.01' stroke='black' stroke-width='0' fill='{}' />", i.0, i.1, color).expect("write error");
        }
    }

    pub fn lines(&mut self, points: &[f64], color: &str) {
        for (a, b) in points.iter()
                            .tuples::<(_, _)>()
                            .tuple_windows::<(_, _)>()
        {
            writeln!(self.buffer, "<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='{}' stroke-width='0.002' />\n", a.0, b.0, a.1, b.1, color).expect("write error");
        }
    }

    pub fn dashed_lines(&mut self, points: &[f64], color: &str) {
        for (a, b) in points.iter()
                            .tuples::<(_, _)>()
                            .tuple_windows::<(_, _)>()
        {
            writeln!(self.buffer, "<line x1='{}' x2='{}' y1='{}' y2='{}' stroke-dasharray='0.03,0.02' stroke='{}' stroke-width='0.005' />\n", a.0, b.0, a.1, b.1, color).expect("write error");
        }
    }

    pub fn polygon(&mut self, points: &[f64], color: &str) {
        writeln!(self.buffer, "<polygon fill='none' points='").expect("write error");
        for a in points.iter()
            .tuples::<(_, _)>()
        {
            writeln!(self.buffer, "{},{} ", a.0, a.1).expect("write error");
        }
        writeln!(self.buffer, "' stroke='{}' stroke-width='0.002' />", color).expect("write error");
    }
}

impl Default for SVG {
    fn default() -> Self {
        Self::new()
    }
}

pub fn svg(pointset: &[f64], hull: &[f64], name: &str) -> Result<(), io::Error> {
    let mut s = SVG::new();
    s.points(pointset, "black");
    s.polygon(hull, "red");

    let filename = if name.ends_with(".svg") {
        name.to_string()
    } else {
        format!("{}.svg", name)
    };

    s.save(&filename)
}
