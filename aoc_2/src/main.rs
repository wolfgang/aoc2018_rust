use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

fn main()  {
    let ids = load_ids();
    let check_sum = calculate_checksum(&ids);
    let common_chars = get_common_chars(&ids);

    println!("Checksum: {}", check_sum);
    println!("Common chars: {}", common_chars);
}

fn load_ids() -> Vec<String> {
    let mut ids = vec![];

    let f = File::open("input.txt").expect("Failed to open input.txt");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        ids.push(line);
    }

    return ids;
}

fn calculate_checksum(ids: &Vec<String>) -> i32 {
    let mut count_with_two = 0;
    let mut count_with_three = 0;

    for id in ids {
        if contains_two_of_any_letter(&id) {
            count_with_two += 1
        }
        if contains_three_of_any_letter(&id) {
            count_with_three += 1
        }
    }

    return count_with_two*count_with_three;
}

fn get_common_chars(ids: &Vec<String>) -> String {
    for (i, id) in ids.iter().enumerate() {
        for i2 in i+1 .. ids.len() {
            let common = common_chars(&id, &ids[i2]);
            if common.len() == id.len() - 1 {
                return common;
            }
        }
    }

    return String::from("")
}

fn contains_two_of_any_letter(s: &String) -> bool {
    return contains_any_letter_times(2, s);
}

fn contains_three_of_any_letter(s: &String) -> bool {
    return contains_any_letter_times(3, s);
}

fn contains_any_letter_times(wanted_count: i32, s: &String) -> bool {
    let mut counts = HashMap::new();
    for c in s.chars() { 
        if !counts.contains_key(&c) {
            counts.insert(c, 0);
        }
        let count = counts.get_mut(&c).unwrap();
        *count += 1;
    }
    
    for (_, count) in &counts {
        if *count == wanted_count {
            return true;
        }
    }

    return false;
}

fn common_chars(s1: &String, s2: &String) -> String {
    let mut result = String::from("");
    for (i, c) in s1.chars().enumerate() {
        if c == s2.chars().nth(i).unwrap() {
            result.push(c);
        }
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_two_of_any_letter_() {
        assert!(!contains_two_of_any_letter(&String::from("abc")));
        assert!(contains_two_of_any_letter(&String::from("aac")));
        assert!(contains_two_of_any_letter(&String::from("vinihorkulbfedcyzmsqgdxpau")));
        assert!(!contains_two_of_any_letter(&String::from("abbbc")));
    }

    #[test]
    fn contains_three_of_any_letter_() {
        assert!(contains_three_of_any_letter(&String::from("abcdddxx")));
        assert!(!contains_three_of_any_letter(&String::from("abcddddddxx")));
    }

    #[test]
    fn common_chars_() {
        assert_eq!("", common_chars(&String::from("abc"), &String::from("edfg")));
        assert_eq!("ac", common_chars(&String::from("abc"), &String::from("axc")));
        assert_eq!("abcde", common_chars(&String::from("axbcde"), &String::from("aybcde")));
    }

}
