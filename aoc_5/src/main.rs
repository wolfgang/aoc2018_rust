#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

fn main() {
    println!("Hello, world!");
}

fn react_polymer(polymer: &String) -> String {
    let mut result = String::from("");

    // for (i, c) in polymer.char_indices() {
    //     if i>0 && polymer.get(i-1).as_str().to_uppercase() == c.as_str() {
    //         return result;
    //     }
    // }

    return polymer.clone();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_step_reactions() {
        assert_eq!("abcd", react_polymer(&String::from("abcd")));
        // assert_eq!("", react_polymer(&String::from("aA")));
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