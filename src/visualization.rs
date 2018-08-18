use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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
        let mut file = File::create(&path)?;

        self.buffer.push_str("</svg>\n");

        write!(file, "{}", &self.buffer)?;
        Ok(())
    }

    pub fn points(&mut self, pointset: &[f64], color: &str) {
        for i in pointset.iter().tuples::<(_, _)>() {
            self.buffer.push_str(&format!("<circle cx='{}' cy='{}' r='0.01' stroke='black' stroke-width='0' fill='{}' />\n", i.0, i.1, color));
        }
    }

    pub fn lines(&mut self, points: &[f64], color: &str) {
        for (a, b) in points.iter()
                            .tuples::<(_, _)>()
                            .tuple_windows::<(_, _)>()
        {
            self.buffer.push_str(&format!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='{}' stroke-width='0.002' />\n", a.0, b.0, a.1, b.1, color));
        }
    }

    pub fn dashed_lines(&mut self, points: &[f64], color: &str) {
        for (a, b) in points.iter()
                            .tuples::<(_, _)>()
                            .tuple_windows::<(_, _)>()
        {
            self.buffer.push_str(&format!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke-dasharray='0.03,0.02' stroke='{}' stroke-width='0.005' />\n", a.0, b.0, a.1, b.1, color));
        }
    }

    pub fn polygon(&mut self, points: &[f64], color: &str) {
        self.buffer.push_str(&format!("<polygon fill='none' points='"));
        for a in points.iter()
            .tuples::<(_, _)>()
        {
            self.buffer.push_str(&format!("{},{} ", a.0, a.1));
        }
        self.buffer.push_str(&format!("' stroke='{}' stroke-width='0.002' />\n", color));
    }
}

pub fn svg(pointset: &[f64], hull: &[f64], filename: &str) -> Result<(), io::Error> {
    let mut s = SVG::new();
    s.points(pointset, "black");
    s.lines(hull, "red");
    s.save(filename)
}
