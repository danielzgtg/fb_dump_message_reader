extern crate chrono;
extern crate serde_json;
extern crate unicode_segmentation;

use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;
use std::fs;
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
  let filename = env::args().skip(1).next().expect("error getting filename");
  let rawdata = fs::read_to_string(filename).expect("error reading ./data.json");
  let parsed: Value = serde_json::from_str(&rawdata).expect("error parsing json");

  let messages = &parsed["messages"];
  let messages = messages.as_array().expect("error coercing message array");
  if messages.len() == 0 { return }
  let namewidth = messages.iter().map(|x| repair_fb_str(
    x["sender_name"].as_str().expect("expected sender str")
  ).graphemes(true).count()).max().unwrap();
  for x in messages.iter().rev() {
    //let t = &x["type"];
    //if t != "Generic" {
    //  continue;
    //}
    let t = x["timestamp_ms"].as_i64().expect("error parsing timestamp");
    let t = NaiveDateTime::from_timestamp(t / 1000, 0 /*ignore millis*/);
    let t = DateTime::<Utc>::from_utc(t, Utc);
    let t = t.format("%Y-%m-%d %H:%M:%S").to_string();
    let c = x["content"].as_str().unwrap_or("[missing]");
    let c = repair_fb_str(c);
    let s = x["sender_name"].as_str().unwrap();
    let s = repair_fb_str(s);
    println!("[{}] {:0width$}: {}", t, s, c, width = namewidth);
  }
}

fn repair_fb_str(input: &str) -> String {
  let raw = input.chars().map(|x| x as u8).collect::<Vec<u8>>();
  String::from_utf8(raw).expect("failed to repair facebook utf8")
}
