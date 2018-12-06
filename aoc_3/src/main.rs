extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

fn rect_from(rect_spec: &String) -> Rect {
    let re = Regex::new(r"^#\d+\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let caps = re.captures(rect_spec).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let width = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let height = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();

    Rect {x: x, y: y, width: width, height: height}
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

    pub fn add_rect(&mut self, rect: &Rect) {
        for x in rect.x .. rect.x + rect.width {
            for y in rect.y .. rect.y + rect.height {
                let v = self.get(x, y);
                if v < 2 {
                    self.set(x, y, v+1)
                }
            }
        }
    }
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
    fn add_rect_to_grid() {
        let mut grid = Grid::new(3, 3);
        let r1 = Rect {x: 1, y: 1, width: 2, height: 2};
        grid.add_rect(&r1);
        assert_eq!(0, grid.get(0, 0));
        assert_eq!(0, grid.get(1, 0));
        assert_eq!(0, grid.get(2, 0));
        assert_eq!(0, grid.get(0, 1));
        assert_eq!(1, grid.get(1, 1));
        assert_eq!(1, grid.get(2, 1));
        assert_eq!(0, grid.get(0, 2));
        assert_eq!(1, grid.get(1, 2));
        assert_eq!(1, grid.get(2, 2));
    }

    #[test]
    fn add_rect_to_grid_overlap() {
        let mut grid = Grid::new(2, 2);
        let r1 = Rect {x: 0, y: 0, width: 2, height: 1};
        grid.add_rect(&r1);
        assert_eq!(1, grid.get(0, 0));
        assert_eq!(1, grid.get(1, 0));
        assert_eq!(0, grid.get(0, 1));
        assert_eq!(0, grid.get(1, 1));

        grid.add_rect(&r1);
        assert_eq!(2, grid.get(0, 0));
        assert_eq!(2, grid.get(1, 0));
        assert_eq!(0, grid.get(0, 1));
        assert_eq!(0, grid.get(1, 1));

        grid.add_rect(&r1);
        assert_eq!(2, grid.get(0, 0));
        assert_eq!(2, grid.get(1, 0));
        assert_eq!(0, grid.get(0, 1));
        assert_eq!(0, grid.get(1, 1));
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
    fn rect_from_parses_rect_from_string() {
        let rect_spec = String::from("#128 @ 871,217: 11x29");
        let rect = rect_from(&rect_spec);
        assert_eq!(871, rect.x);
        assert_eq!(217, rect.y);
        assert_eq!(11, rect.width);
        assert_eq!(29, rect.height);
    }

}
