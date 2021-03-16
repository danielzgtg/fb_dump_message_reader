use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use serde_json::Value;

use fb_dump_message_reader::{convert, Stats};

fn main() {
    let parsed_path: &Path = "parsed".as_ref();
    fs::create_dir(parsed_path).expect("failed to create output dir, already exists?");
    let mut read_buf = String::with_capacity(100_000);
    let mut write_buf = String::with_capacity(100_000);
    let mut stats = Stats::new();
    for entry in fs::read_dir("messages").expect("couldn't read dumped messages folder") {
        let entry = entry.expect("failed to read messages folder entry");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        for entry in path.read_dir().expect("couldn't read type folder") {
            let entry = entry.expect("failed to read type folder entry");
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = entry.file_name();
            let name = name.as_os_str().to_str().expect("corrupt thread name");
            for entry in path.read_dir().expect("couldn't read thread folder") {
                let entry = entry.expect("failed to read thread folder entry");
                let path = entry.path();
                let file_name = entry.file_name();
                if file_name != "message_1.json" {
                    if let Some(s) = file_name.to_str() {
                        if s.contains("json") {
                            eprintln!("Warning: Unknown json file");
                        }
                    }
                    continue;
                }
                assert!(path.is_file(), "message_1.json should be a file!");
                read_buf.clear();
                write_buf.clear();
                let mut f = File::open(path).expect("error opening thread");
                f.read_to_string(&mut read_buf).expect("error reading thread");
                let parsed: Value = serde_json::from_str(&read_buf).expect("error parsing json");
                convert(&parsed, &mut write_buf, &mut stats);
                let target_path = parsed_path.join(format!("{}.txt", name));
                let mut f = OpenOptions::new().write(true).create_new(true).open(target_path)
                    .expect("error creating result file");
                f.write((&write_buf).as_ref()).expect("error writing result");
            }
        }
    }
    stats.print();
}
