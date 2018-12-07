
extern crate regex;

use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

fn main() {
    println!("Reading rects ..");
    let (rects, max_x, max_y) = read_rects();

    let grid = create_grid(&rects, max_x, max_y);

    println!("Calculating answer for part 1 ..");
    let sum = get_number_of_common_inches(&grid);
    println!("Calculating answer for part 2 ..");
    let non_overlapping_id = get_non_overlapping_id(&rects, &grid);

    assert_eq!(121259, sum, "Wrong answer for part 1");
    assert_eq!(239, non_overlapping_id, "Wrong answer for part 2");

    println!("SUCCESS!");
}

fn read_rects() -> (Vec<(usize, Rect)>, usize, usize) {
    let mut rects = vec![];

    let mut max_x = 0;
    let mut max_y = 0;


    let f = File::open("input.txt").expect("Failed to open input.txt");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        let (id, rect) = rect_from(&line);
        if &rect.x + &rect.width > max_x {
            max_x = &rect.x + &rect.width;
        }

        if &rect.y + &rect.height > max_y {
            max_y = &rect.y + &rect.height;
        }
        rects.push((id, rect));

    }

    (rects, max_x, max_y)
}


fn create_grid(rects: &Vec<(usize, Rect)>, grid_width: usize, grid_height: usize) -> Grid {

    let mut grid = Grid::new(grid_width, grid_height);
    for (_, rect) in rects {
        grid.add_rect(&rect)
    }

    return grid;
}

fn get_number_of_common_inches(grid: &Grid) -> usize {
    let mut sum = 0;

    for x in 0 .. grid.width {
        for y in 0 .. grid.height {
            if grid.get(x, y) == 2 {
                sum += 1;
            }
        }
    }
    return sum;
}

fn get_non_overlapping_id(rects: &Vec<(usize, Rect)>, grid: &Grid) -> usize {
    for (id, rect) in rects {
        if !grid.has_rect_overlaps(&rect) {
            return *id;
        }
    }
    return 0;
}

struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

fn rect_from(rect_spec: &String) -> (usize, Rect) {
    let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let caps = re.captures(rect_spec).unwrap();
    let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let width = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let height = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

    (id, Rect {x: x, y: y, width: width, height: height})
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

    pub fn has_rect_overlaps(&self, rect: &Rect) ->bool {
        for x in rect.x .. rect.x + rect.width {
            for y in rect.y .. rect.y + rect.height {
                if  self.get(x, y) == 2 {
                    return true
                }
            }
        }

        return false;
    }
}


#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

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
    fn has_rect_overlaps() {
        let mut grid = Grid::new(2, 2);
        let r1 = Rect {x: 0, y: 0, width: 2, height: 1};
        assert!(!grid.has_rect_overlaps(&r1));
        grid.add_rect(&r1);
        assert!(!grid.has_rect_overlaps(&r1));
        let r2 = Rect {x: 0, y: 0, width: 1, height: 1};
        grid.add_rect(&r2);
        assert!(grid.has_rect_overlaps(&r1));
        assert!(grid.has_rect_overlaps(&r2));
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
        let (id, rect) = rect_from(&rect_spec);
        assert_eq!(128, id);
        assert_eq!(871, rect.x);
        assert_eq!(217, rect.y);
        assert_eq!(11, rect.width);
        assert_eq!(29, rect.height);
    }
}
