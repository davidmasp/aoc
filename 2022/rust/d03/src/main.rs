

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
    
    //println!("letter: {}", letter);

    let result: &u32 = match prio.get(letter) {
        Some(int_val) => int_val,
        None => panic!("Wrong Guess value")
    };
    return *result
}

#[derive(Debug, Clone)]
struct Elf {
    backpack: String,
}


struct ElfGroup<'a> {
    elf1: &'a Elf,
    elf2: &'a Elf,
    elf3: &'a Elf,
}

fn get_prio(elf: &Elf) -> u32 {
    let mut line_split: Vec<&str> = elf.backpack.split("").collect();
    line_split.remove(0);
    line_split.remove(line_split.len()-1);
    let mid_len: usize = ((line_split.len() as u32 / 2)).try_into().unwrap();
    let vec2 = line_split.split_off(mid_len);
    //println!("{:?}", vec2);
    let common_letter = get_common_letter(&line_split, &vec2);
    let priority: u32 = get_score_letter(common_letter);
    //rectangle.width * rectangle.height
    return priority
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

fn get_badge_prio(eg: &ElfGroup) -> u32{

    // this is extremely complicated because I had a stupid error in split
    // that I was not seeing...
    // now I am lazy to go back
    //println!("elf1: {}", eg.elf1.backpack);
    let mut ex: Vec<&str> = eg.elf1.backpack.split("").collect();
    ex.remove(0);
    ex.remove(ex.len()-1);
    let exl: usize = ex.len();
    ////
    let mut ey: Vec<&str> = eg.elf2.backpack.split("").collect();
    ey.remove(0);
    ey.remove(ey.len()-1);
    let eyl: usize = ey.len();
    ////
    let mut ez: Vec<&str> = eg.elf3.backpack.split("").collect();
    ez.remove(0);
    ez.remove(ez.len()-1);
    let ezl: usize = ez.len();

    let mut common_letter: &str = "";
    let mut idxx: usize = 0;
    while idxx < exl {
        let mut idxy: usize = 0;
        while idxy < eyl {
            let mut idxz: usize = 0;
            while idxz < ezl {
                if ex[idxx] == ey[idxy] && ey[idxy] == ez[idxz] {
                    common_letter = ex[idxx];
                }
                idxz += 1;
            }
            idxy += 1;
        }
        idxx += 1;
    }
    
    //println!("common letter: {}", common_letter);

    let prio: u32 = get_score_letter(&common_letter);
    return prio
}

fn main() {
    
    // let mut total_score: u32 = 0;
    //let mut total_score2: u32 = 0;
    // let mut counter: u32 = 0;

    let mut all_elves: Vec<Elf> = Vec::new();
    let mut all_elves2: Vec<Elf> = Vec::new();

    // gets the stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();

        let elf_element: Elf = Elf {
            backpack: this_line,
        };
        // i dont think this is the rust-way to do it
        let inst: Elf = elf_element.clone();
        all_elves.push(inst);
        let inst2: Elf = elf_element.clone();
        all_elves2.push(inst2);
    }

    let total_len: usize = all_elves.len();
    let mut total_score: u32 = 0;
    let mut idx: usize = 0;
    while idx < total_len {
        let priority: u32 = get_prio(&all_elves[idx]);
        total_score += priority;
        idx += 1;
    }

    idx = 0;
    let mut total_score2: u32 = 0;
    while idx < total_len {
        let eg: ElfGroup = ElfGroup {
            elf1: &all_elves2[idx],
            elf2: &all_elves2[idx + 1],
            elf3: &all_elves2[idx + 2],
        };

        let priority2: u32 = get_badge_prio(&eg);
        total_score2 += priority2;

        idx += 3;
    } 
    println!("total score is {}", total_score);
    println!("total score2 is {}", total_score2);
    
}

        
/* let priority: u32 = get_prio(&elf_element);
total_score += priority;

counter += 1;
if counter == 1 {
    eg1.elf1 = &elf_element;
} else if counter == 2 {
    eg1.elf2 = &elf_element;
} else if counter == 3 {
    eg1.elf3 = &elf_element;
    let priority2: u32 = get_badge_prio(&eg1);
    total_score2 += priority2;
    counter = 0;
}      */   
