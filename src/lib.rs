use std::io::Write;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

fn repair_fb_str(input: &str) -> String {
    let raw = input.chars().map(|x| x as u8).collect::<Vec<u8>>();
    String::from_utf8(raw).expect("failed to repair facebook utf8")
}

pub struct Stats {
    early: DateTime<Utc>,
    late: DateTime<Utc>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            early: Utc::now(),
            late: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        }
    }

    pub fn print(&self) {
        println!("From {} to {}", self.early, self.late);
    }

    fn offer(&mut self, time: &DateTime<Utc>) {
        if *time < self.early {
            self.early = time.clone();
        }
        if *time > self.late {
            self.late = time.clone();
        }
    }
}

pub fn convert(parsed: &Value, out: &mut impl Write, stats: &mut Stats) {
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
        stats.offer(&t);
        let t = t.format("%Y-%m-%d %H:%M:%S").to_string();
        let c = x["content"].as_str().unwrap_or("[missing]");
        let c = repair_fb_str(c);
        let s = x["sender_name"].as_str().unwrap();
        let s = repair_fb_str(s);
        writeln!(out, "[{}] {:0w$}: {}", t, s, c, w = namewidth).unwrap();
    }
}