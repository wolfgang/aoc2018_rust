extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

struct Grid {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<i32>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid { width: width, height: height, elements: vec!(0; width*height) }
    }

    pub fn set(&mut self, x: usize, y: usize, value: i32)  {
        self.elements[x + y * self.width] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.elements[x + y*self.width]
    }
}

struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

fn rect_from(s: &String) -> Rect {
    let rect_part = s.split("@").collect::<Vec<&str>>()[1].trim();

    let split2 = rect_part.split(",").collect::<Vec<&str>>();
    let x = split2[0].parse::<usize>().unwrap();

    Rect {x: x, y: 0, width: 0, height: 0}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid() {
        let mut g = Grid::new(10, 20);
        g.set(5, 6, 1234);
        assert_eq!(1234, g.get(5, 6));
    }

    #[test]
    fn rect_struct() {
        let r = Rect {x: 1, y: 2, width: 3, height: 4};
        assert_eq!(1, r.x);
        assert_eq!(2, r.y);
        assert_eq!(3, r.width);
        assert_eq!(4, r.height,);
    }

    #[test]
    fn split_and_trim_strings() {
        let s = "#1200 @ 94,536: 22x13";
        let split: Vec<&str> = s.split("@").collect();
        assert_eq!("#1200 ", split[0]);
        assert_eq!(" 94,536: 22x13", split[1]);
        assert_eq!("#1200", split[0].trim());
        assert_eq!("94,536: 22x13", split[1].trim());
    }

    #[test]
    fn regex() {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert!(re.is_match("2014-01-01"));

        let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
        let caps = re.captures("abc123").unwrap();

        let text1 = caps.get(1).unwrap().as_str();
        assert_eq!(text1, "123");
    }


    #[test]
    fn parse_rect_from_string() {
        let s1 = String::from("#1200 @ 94,536: 22x13");
        let parsed_rect = rect_from(&s1);

        assert_eq!(94, parsed_rect.x);
        // assert_eq!(536, parsed_rect.y);
        // assert_eq!(22, parsed_rect.width);
        // assert_eq!(13, parsed_rect.height);

    }
}
