pub mod atlas;
use crate::atlas::AtlasParser;
use crate::atlas::ConcreteAtlasParser;

use std::fs;

fn main() {
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./hosts") {
    //     // Consumes the iterator, returns an (Optional) String
    //     let lines = lines.map(|v| v.unwrap().as_str());

    //     println!("{:?}", atlas);
    // }

    let v = fs::read_to_string("example/example.atlas").unwrap();
    let super_inefficient_nooby_lines: Vec<_> = v.split("\n").collect();
    let parser = ConcreteAtlasParser {};
    let atlas = parser.parse(super_inefficient_nooby_lines.iter());
    println!("{:#?}", atlas);
}
