extern crate serde_json;

use std::fs;
use serde_json::Value;

fn main() {
  let rawdata = fs::read_to_string("./message_1.json").expect("error reading ./data.json");
  let parsed: Value = serde_json::from_str(&rawdata).expect("error parsing json");

  let messages = &parsed["messages"];
  for x in messages.as_array().expect("error coercing message array").iter().rev() {
    //let t = &x["type"];
    //if t != "Generic" {
    //  continue;
    //}
    let c = &x["content"];
    let s = &x["sender_name"];
    println!("{}: {}", s, c);
  }
}
