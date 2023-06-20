// day 6

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn check_unique(x: &Vec<String>) -> bool {
    let mut stream:HashSet<String> = HashSet::new();
    for i in x{
        stream.insert(i.to_string());
    }
    if x.len() == stream.len() {
        return true
    } else {
        return false
    }
}

fn check_unique_seq(x: &str, k_val: usize) -> i32 {
    let mut stream_vec: Vec<String> = Vec::new();
    let mut counter: i32= 0;
    for c in x.chars() {
        counter += 1 ;// becuase 1-based goes first
        let str_ele: String = c.to_string(); 
        if stream_vec.len() == k_val {
            // remove the first element
            stream_vec.remove(0);
            stream_vec.push(str_ele);
            if check_unique(&stream_vec){
                break
            }
        } else {
            stream_vec.push(str_ele);
            continue
        }
    }
    return counter;
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let k1: usize = 4;
        let k2: usize = 14;
        let sol1 = check_unique_seq(&this_line, k1);
        println!("Position of marker: {}", sol1);
        let sol2 = check_unique_seq(&this_line, k2);
        println!("Position of the mess: {}", sol2);
    }
    
}
