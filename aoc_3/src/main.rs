use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut count_with_two = 0;
    let mut count_with_three = 0;
    for line in f.lines() {
        let line = line.unwrap();
        if contains_two_of_any_letter(&line) {
        	count_with_two += 1
        }
        if contains_three_of_any_letter(&line) {
        	count_with_three += 1
        }
    }

    println!("Checksum: {}", count_with_two*count_with_three);

    Ok(())
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


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn contains_two_of_any_letter_only_singles() {
        assert!(!contains_two_of_any_letter(&String::from("abc")));
    }

    #[test]
    fn contains_two_of_any_letter_exactly_2_of_one_letter() {
        assert!(contains_two_of_any_letter(&String::from("aac")));
    }

    #[test]
    fn contains_two_of_any_letter_exactly_2_of_more_letters() {
        assert!(contains_two_of_any_letter(&String::from("vinihorkulbfedcyzmsqgdxpau")));
    }

    #[test]
    fn contains_two_of_any_letter_exactly_more_than_2() {
        assert!(!contains_two_of_any_letter(&String::from("abbbc")));
    }

    #[test]
    fn contains_three_of_any_letter_exactly_3() {
        assert!(contains_three_of_any_letter(&String::from("abcdddxx")));
    }

    #[test]
    fn contains_three_of_any_letter_more_than_three() {
        assert!(!contains_three_of_any_letter(&String::from("abcddddddxx")));
    }

}
