#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let polymer = read_input();

    println!("Calculating part 1 answer ..");
    assert_eq!(9154, get_length_of_reduced(&polymer));
    println!("Calculating part 2 answer (this takes a bit) ..");
    assert_eq!(4556, get_length_of_shortest(&polymer));

    println!("SUCCESS!!");
}

fn read_input() -> String {
    let mut f = File::open("input.txt").expect("Failed to open input.txt");
    let mut polymer = String::new();
    f.read_to_string(&mut polymer).expect("could not read contents of input.txt");
    return polymer;
}

fn get_length_of_reduced(polymer: &String) -> usize {
    let reduced = reduce_polymer(&polymer);
    return reduced.len();
}

fn get_length_of_shortest(polymer: &String) -> usize {
    let mut min_len = polymer.len();
    for unit_as_byte in 'a' as u8 .. 'z' as u8 {
        let unit = unit_as_byte as char;
        let polymer_stripped = remove_unit(unit, &polymer);
        let reduced = reduce_polymer(&polymer_stripped);
        if reduced.len() < min_len { min_len = reduced.len(); }
    }

    return min_len;
}

fn reduce_polymer(polymer: &String) -> String {
    let mut reduced = polymer.to_string();
    let mut previous_length = polymer.len();
    loop {
        reduced = react_polymer(&reduced);
        if reduced.len() == previous_length { break; }
        previous_length = reduced.len();
    }
    return reduced;
}

fn react_polymer(polymer: &String) -> String {
    let mut result = String::with_capacity(polymer.len());
    let mut i = 0;
    while i < polymer.len() {
        let current_char = polymer.as_bytes()[i] as char;
        if i < polymer.len() - 1 {
            let next_char = polymer.as_bytes()[i+1] as char;
            if is_reacting(current_char, next_char) {
                i += 2;
                continue;
            }
        }
        result.push(current_char);
        i += 1;
    }

    return result;
}

fn is_reacting(c1: char, c2: char) -> bool {
    return c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase();
}

fn remove_unit(unit: char, polymer: &String) -> String {
    let mut result = String::with_capacity(polymer.len());
    let mut i = 0;
    while i < polymer.len() {
        let current_char = polymer.as_bytes()[i] as char;
        if current_char.to_ascii_lowercase() != unit {
            result.push(current_char)
        }
        i += 1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_step_reactions() {
        assert_eq!("abcd", react_polymer(&String::from("abcd")));
        assert_eq!("", react_polymer(&String::from("aA")));
        assert_eq!("aa", react_polymer(&String::from("aa")));
        assert_eq!("", react_polymer(&String::from("Aa")));
        assert_eq!("abcd", react_polymer(&String::from("abxXcd")));
        assert_eq!("abcd", react_polymer(&String::from("abcdSs")));
        assert_eq!("abcd", react_polymer(&String::from("Uuabcd")));
     }

     #[test]
     fn reduce_polymer_() {
        let polymer = String::from("dabAcCaCBAcCcaDA");
        assert_eq!("dabCBAcaDA", reduce_polymer(&polymer));

     }

     #[test]
     fn remove_unit_() {
        let polymer = String::from("dabAcCaCBAcCcaDA");
        assert_eq!("dbcCCBcCcD", remove_unit('a', &polymer));
     }

    #[test]
    fn strings() {
        let s = String::from("abcd");
        assert_eq!('a', s.chars().nth(0).unwrap());
        assert_eq!('d', s.chars().nth(3).unwrap());
        assert_eq!(2, "ab".len());
    }

    #[test]
    fn chars() {
        assert_eq!("a", 'a'.to_string());
        assert_eq!('A', 'a'.to_ascii_uppercase());
        assert_eq!('b', 'B'.to_ascii_lowercase());
    }

    #[test]
    fn string_as_bytes() {
        let s = String::from("abcd");
        let b = s.as_bytes();
        assert_eq!('a', b[0] as char);
        assert_eq!('b', b[1] as char);
    }
}
