#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

fn main() {
    println!("Hello, world!");
}

fn react_polymer(polymer: &String) -> String {
    let result = String::from("");

    for (i, c) in polymer.char_indices() {
        if i>0 {
            let prev_char = polymer.chars().nth(i-1).unwrap();
            if is_reacting(prev_char, c) {
                return result;
            }
        }
    }

    return polymer.clone();
}

fn is_reacting(c1: char, c2: char) -> bool {
    return c1 != c2 && (c1.to_ascii_lowercase() == c2 || c1.to_ascii_uppercase() == c2);
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
     }

    #[test]
    fn indexing_strings() {
        let s = String::from("abcd");
        assert_eq!('a', s.chars().nth(0).unwrap());
        assert_eq!('d', s.chars().nth(3).unwrap());
    }

    #[test]
    fn chars() {
        assert_eq!("a", 'a'.to_string());
        assert_eq!('A', 'a'.to_ascii_uppercase());
        assert_eq!('b', 'B'.to_ascii_lowercase());
    }
}