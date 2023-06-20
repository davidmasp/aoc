

// I assume the folder names and file names are unique...
// I had to get inspiration from here as I was a bit stuck in this one
// https://github.com/SamStudio8/adventofcode/blob/main/2022/day_7/src/main.rs


use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn parse_line(x: String) -> (String, String, String) {
    let cdre = Regex::new(r"\$ (\w+)\s*(.*)").unwrap();
    let dirre = Regex::new(r"(\D+) (.)").unwrap();
    let filere =  Regex::new(r"(\d+) (.)").unwrap();
    let mut line_type: String = "".to_string();
    let mut c1: String = "".to_string();
    let mut c2: String = "".to_string(); 
    // tmp
    if cdre.is_match(&x){
        let caps = cdre.captures(&x).unwrap();
        //println!("{:?}", caps);
        //println!("{} - {}", &x, "CD string");
        line_type = "cmd".to_string();
        c1 = caps[1].to_string();
        c2 = caps[2].to_string();
    } else if dirre.is_match(&x){
        let caps = dirre.captures(&x).unwrap();
        //println!("{:?}", caps);
        //println!("{} - {}", &x, "DIR string");
        line_type = "dir".to_string();
        c1 = caps[1].to_string();
        c2 = caps[2].to_string();
    } else if filere.is_match(&x){
        let caps = filere.captures(&x).unwrap();
        //println!("{:?}", caps);
        //println!("{} - {}", &x, "FILE string");
        line_type = "file".to_string();
        c1 = caps[1].to_string();
        c2 = caps[2].to_string();
    }
    
    return (line_type, c1, c2);
}

fn main() {
    let mut fs = HashMap::new();
    let mut cwd: String = "".to_string();
    // up co 100000
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let (ctype, f1, f2) = parse_line(this_line.to_string());
        // this should happen once, can change if twice
        if ctype == "cmd" && &f1 == "cd" {
            if &f2 == "/" {
                cwd = "/".to_string();
            } else if &f2 == ".." {
                cwd = cwd.rsplit_once('/').unwrap().0.to_string();
            } else {
                cwd = format!("{}/{}", cwd, &f2);
            }
            // this inserts an entry if it does not exist
            let cwd_clone = cwd.clone();
            fs.entry(cwd_clone).or_insert(0);
        } else if ctype == "cmd" && f1 == "ls"{
            // this is ls, no need
            continue
        } else if ctype == "dir" {
            // this is dir, no need
            continue
        } else {
            // this is a listed file   
            let size: i32 = f1.parse::<i32>().unwrap();
            for (key, val) in fs.iter_mut() {
                let key_str: String = key.to_string();
                if cwd.starts_with(&key_str){
                    // idk why *, i think you need to dereference because iterator.
                    *val += size;
                }
            }
        }
    }
    let mut total_sum = 0;
    let cut_off = 100000;
    for (_, vsize) in fs.iter(){
        if vsize<&cut_off{
            total_sum+=vsize;
        }
    }
    println!("Total sum: {}", total_sum);
    let total_size: i32 = *fs.get("/").unwrap();
    println!("Total size: {}", total_size);
    let total_size_available = 70000000;
    let total_needed_size = 30000000;
    let unused_size = total_size_available - total_size;
    let needed_size = total_needed_size - unused_size;
    println!("Needed size: {}", needed_size);
    let mut all_sizes: Vec<i32> = fs.into_values().collect();
    all_sizes.sort();
    for i in all_sizes{
        if i > needed_size{
            println!("Size to delete: {}", i);
            break
        }
    }

}
