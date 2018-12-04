use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn contains_two_of_any_letter(s: String) -> bool {
	return contains_any_letter_times(2, s);
}

fn contains_three_of_any_letter(s: String) -> bool {
	return contains_any_letter_times(3, s);
}

fn contains_any_letter_times(wanted_count: i32, s: String) -> bool {
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
        assert!(!contains_two_of_any_letter("abc".into()));
    }

    #[test]
    fn contains_two_of_any_letter_exactly_2_of_one_letter() {
        assert!(contains_two_of_any_letter("aac".into()));
    }

    #[test]
    fn contains_two_of_any_letter_exactly_2_of_more_letters() {
        assert!(contains_two_of_any_letter("abccddee".into()));
    }

    #[test]
    fn contains_two_of_any_letter_exactly_more_than_2() {
        assert!(!contains_two_of_any_letter("abbbc".into()));
    }

    #[test]
    fn contains_three_of_any_letter_exactly_3() {
        assert!(contains_three_of_any_letter("abcdddxx".into()));
    }

    #[test]
    fn contains_three_of_any_letter_more_than_three() {
        assert!(!contains_three_of_any_letter("abcddddddxx".into()));
    }

}
