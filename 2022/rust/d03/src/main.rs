

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

//day 3
fn get_score_letter(letter: &str) -> u32 {
    let prio = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52),
    ]);
    let result: &u32 = match prio.get(letter) {
        Some(int_val) => int_val,
        None => panic!("Wrong Guess value")
    };
    return *result
}

fn get_common_letter<'a>(lx: &'a Vec<&str>, ly: &'a Vec<&str>) -> &'a str {
    let mut common_letter: &str = "";
    for x in lx {
        for y in ly {
            if x == y {
                common_letter = x;
            }
        }
    }
    return common_letter
}

fn main() {
    println!("Hello, world!");
    println!("prio of B is {}", get_score_letter("B"));

    let mut total_score: u32 = 0;

    // gets the stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let mut line_split: Vec<&str> = this_line.split("").collect();
        let mid_len: usize = ((line_split.len() as u32 / 2)).try_into().unwrap();
        let vec2 = line_split.split_off(mid_len);
        let common_letter = get_common_letter(&line_split, &vec2);
        let priority: u32 = get_score_letter(common_letter);
        total_score += priority;
    }

    println!("total score is {}", total_score);
}
