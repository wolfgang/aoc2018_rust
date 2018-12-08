#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

fn main() {
    println!("Hello, world!");
}

fn react_polymer(polymer: &String) -> String {
    let mut result = String::from("");

    let mut i = 0;

    while i < polymer.len() {
        let current_char = char_at(i, polymer);
        if i < polymer.len() - 1 {
            let next_char = char_at(i+1, polymer);
            if is_reacting(current_char, next_char) {
                i +=2;
                continue;
            }
        }
        result.push(current_char);
        i += 1;
    }

    return result;
}

fn char_at(index: usize, s: &String) -> char {
    s.chars().nth(index).unwrap()
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
        assert_eq!("abcd", react_polymer(&String::from("abxXcd")));
        assert_eq!("abcd", react_polymer(&String::from("abcdSs")));
        assert_eq!("abcd", react_polymer(&String::from("Uuabcd")));

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
}