
// day 4

use std::io;
use std::io::prelude::*;

struct Interval {
    start: u32,
    end: u32,
}

fn build_interval(x: &str) -> Interval {
    let mut interval: Interval = Interval {start: 0, end: 0};
    let split: Vec<&str> = x.split("-").collect();
    interval.start = split[0].parse::<u32>().unwrap();
    interval.end = split[1].parse::<u32>().unwrap();
    return interval
}

// the small interval is the x
fn fully_enclosed(x: &Interval, y: &Interval) -> bool {
    if x.start >= y.start && x.end <= y.end {
        return true
    }
    return false
}

// the small interval is the x
fn do_overlap(x: &Interval, y: &Interval) -> bool {
    if x.start >= y.start && x.start <= y.end {
        // right overlap
        return true
    } else if x.end >= y.start && x.end <= y.end {
        // left overlap
        return true
    } else if fully_enclosed(x, y){
        return true
    }
    return false
}

fn main() {

    let mut total_score: u32 = 0;
    let mut total_overlap: u32 = 0;

    // gets the stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let line_split: Vec<&str> = this_line.split(",").collect();
        let interval1 = build_interval(line_split[0]);
        let interval2 = build_interval(line_split[1]);

        let enclosed1: bool = fully_enclosed(&interval1, &interval2);
        let enclosed2: bool = fully_enclosed(&interval2, &interval1);

        if enclosed1 || enclosed2 {
            total_score += 1;
        }

        let overlap: bool = do_overlap(&interval1, &interval2);
        let overlap2: bool = do_overlap(&interval2, &interval1);
        if overlap || overlap2 {
            total_overlap += 1;
        }
    }    
    println!("Total enclosed:{}", total_score);
    println!("Total overlap:{}", total_overlap);
}
