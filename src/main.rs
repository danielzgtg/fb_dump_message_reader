use std::env;
use std::fs;

use serde_json::Value;

use fb_dump_message_reader::{convert, Stats};

fn main() {
    let filename = env::args().skip(1).next().expect("error getting filename");
    let rawdata = fs::read_to_string(filename).expect("error reading ./data.json");
    let parsed: Value = serde_json::from_str(&rawdata).expect("error parsing json");
    let mut stats = Stats::new();
    convert(&parsed, &mut std::io::stdout(), &mut stats);
    stats.print();
}
