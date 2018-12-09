#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate regex;


use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

type Coord = (i32, i32);


fn main() {
    let coords = read_coords();
    println!("Calculating part 1 answer ..");
    assert_eq!(3660, find_largest_finite_area(&coords));
    println!("SUCCESS!!");

}

fn read_coords() -> Vec<Coord> {
    let mut input = vec![];

    let f = File::open("input.txt").expect("Failed to open input.txt");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        input.push(parse_coords(&line));
    }

    return input;
}

fn find_largest_finite_area(coords: &Vec<Coord>) -> usize {
    let (max_x, max_y) = find_max_x_y(coords);
    let ar = build_areas(&coords, max_x, max_y);

    let mut max_size = 0;

    for (_, area) in ar.areas {
        if !area.is_infinite(max_x, max_y) && area.size() > max_size {
            max_size = area.size();
        }
    }

    return max_size;
}

fn find_max_x_y(coords: &Vec<Coord>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in coords {
        if *x > max_x { max_x = *x; }
        if *y > max_y { max_y = *y; }
    }

    return (max_x, max_y);
}


fn parse_coords(s: &String) -> Coord {
    let re = Regex::new(r"(\d+),\s(\d+)").unwrap();
    let caps = re.captures(s).unwrap();
    let x = caps[1].parse::<i32>().unwrap();
    let y = caps[2].parse::<i32>().unwrap();

    return (x, y)
}

fn md(c1 : Coord, c2: Coord) -> u32 {
    let (x1, y1) = c1;
    let (x2, y2) = c2;

    return ((x2 - x1).abs() + (y2 - y1).abs()) as u32;
}

#[derive(Debug, PartialEq)]
struct Area {
    coordinates: Vec<Coord>
}

impl Area {
    fn new() -> Area {
        Area {coordinates: Vec::new()}
    }

    fn add_coordinate(&mut self, coord: Coord) {
        self.coordinates.push(coord);
    }

    fn size(&self) -> usize {
        return self.coordinates.len();
    }

    fn is_infinite(&self, max_x: i32, max_y: i32) -> bool {
        let b = self.coordinates.iter().any(
            |(x, y)| *x == 0 || *y == 0 || *x == max_x || *y == max_y);
        return b;
    }
}

struct AreaRegistry {
    areas: HashMap<Coord, Area>
}

impl AreaRegistry {
    fn new() -> AreaRegistry {
        AreaRegistry { areas: HashMap::new() }
    }

    fn add_coord_to_area(&mut self, center: Coord, coord: Coord) {
        let area = self.areas.entry(center).or_insert(Area::new());
        area.add_coordinate(coord);
    }
}

fn build_areas(coords: &Vec<Coord>, max_x: i32, max_y: i32) -> AreaRegistry {
    let mut ar = AreaRegistry::new();

    for y in 0 ..= max_y {
        for x in 0 ..= max_x {
            let nearest = find_nearest_coordinates_of(x, y, coords);
            if nearest.len()==1 {
                ar.add_coord_to_area(nearest[0], (x, y))
            }
        }
    }

    return ar;
}

fn find_nearest_coordinates_of(x: i32, y: i32, coordinates: &Vec<Coord>) -> Vec<Coord> {
    let mut all_nearest : Vec<(u32, Coord)> = Vec::new();
    let mut nearest_distance = 99999;
    for coord in coordinates {
        if md((x, y), *coord) <= nearest_distance {
            nearest_distance = md((x, y), *coord);
            all_nearest.push((nearest_distance, *coord));
        }
    }

    return all_nearest.into_iter()
                .filter(|(dist, _)| dist==&nearest_distance)
                .map(|(_, coord)| coord)
                .collect();
}

fn total_distance_from(x: i32, y: i32, coordinates: &Vec<Coord>) -> u32 {
    let mut result = 0;
    for coord in coordinates {
        result += md((x, y), *coord);
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coords_()  {
        assert_eq!((257, 332), parse_coords(&String::from("257, 332")));
        assert_eq!((1, 2), parse_coords(&String::from("1, 2")));
        assert_eq!((100, 2), parse_coords(&String::from("100, 2")));
        assert_eq!((1, 200), parse_coords(&String::from("1, 200")));
    }

    #[test]
    fn manhatten_distance() {
        assert_eq!(0,  md((0, 0), (0, 0)));
        assert_eq!(31,  md((1, 2), (14, 20)));
        assert_eq!(31,  md((14, 20), (1, 2)));
    }

    #[test]
    fn initialize_area() {
        let area = Area::new();
        assert_eq!(0, area.coordinates.len());
    }

    #[test]
    fn add_coords_to_area() {
        let mut area = Area::new();
        area.add_coordinate((5, 6));
        area.add_coordinate((7, 8));
        assert_eq!(vec![(5, 6), (7, 8)], area.coordinates);
    }

    #[test]
    fn area_is_infinite() {
        let mut area1 = Area::new();
        area1.add_coordinate((1, 1));
        area1.add_coordinate((2, 2));
        assert!(!area1.is_infinite(10, 10));

        let mut area2 = Area::new();
        area2.add_coordinate((0, 1));
        area2.add_coordinate((2, 2));
        assert!(area2.is_infinite(10, 10));

        let mut area3 = Area::new();
        area3.add_coordinate((1, 1));
        area3.add_coordinate((2, 0));
        assert!(area3.is_infinite(10, 10));

        let mut area4 = Area::new();
        area4.add_coordinate((1, 1));
        area4.add_coordinate((10, 2 ));
        assert!(area4.is_infinite(10, 10));

        let mut area5 = Area::new();
        area5.add_coordinate((1, 1));
        area5.add_coordinate((1, 10 ));
        assert!(area5.is_infinite(10, 10));

    }

    #[test]
    fn initialize_area_registry() {
        let ar = AreaRegistry::new();
        assert_eq!(0, ar.areas.len());
    }

    #[test]
    fn add_coord_to_area() {
        let mut ar = AreaRegistry::new();
        ar.add_coord_to_area((1, 2), (3, 4));
        ar.add_coord_to_area((1, 2), (5, 6));
        assert_eq!(1, ar.areas.len());
        let area = ar.areas.get(&(1, 2)).unwrap();
        assert_eq!(vec![(3,4), (5, 6)], area.coordinates);
    }

    #[test]
    fn build_areas_() {
        let coords = vec![(0, 0), (2, 2)];
        let ar = build_areas(&coords, 2, 2);

        assert_eq!(2, ar.areas.keys().len());
        assert!(ar.areas.contains_key(&(0, 0)));
        assert!(ar.areas.contains_key(&(2, 2)));
        let area00 = ar.areas.get(&(0, 0)).unwrap();
        assert_eq!(3, area00.size());
        assert!(area00.coordinates.contains(&(0, 0)));
        assert!(area00.coordinates.contains(&(1, 0)));
        assert!(area00.coordinates.contains(&(0, 1)));

        let area22 = ar.areas.get(&(2, 2)).unwrap();
        assert_eq!(3, area00.size());
        assert!(area22.coordinates.contains(&(1, 2)));
        assert!(area22.coordinates.contains(&(2, 1)));
        assert!(area22.coordinates.contains(&(2, 2)));
    }

    #[test] 
    fn find_nearest_coordinates_of_a_point() {
        let coords = vec![(0, 0), (2, 2)];

        let nearest = find_nearest_coordinates_of(0, 0, &coords);
        assert_eq!(1, nearest.len());
        assert_eq!((0, 0), nearest[0]);

        let nearest = find_nearest_coordinates_of(1, 0, &coords);
        assert_eq!(1, nearest.len());
        assert_eq!((0, 0), nearest[0]);

        let nearest = find_nearest_coordinates_of(0, 1, &coords);
        assert_eq!(1, nearest.len());
        assert_eq!((0, 0), nearest[0]);

        let nearest = find_nearest_coordinates_of(2, 1, &coords);
        assert_eq!(1, nearest.len());
        assert_eq!((2, 2), nearest[0]);

        let nearest = find_nearest_coordinates_of(2, 0, &coords);
        assert_eq!(2, nearest.len());
        assert_eq!((0, 0), nearest[0]);
        assert_eq!((2, 2), nearest[1]);
    }

    #[test]
    fn total_distance_from_() {
        let coords = vec![(0, 0), (2, 0), (2, 2)];
        assert_eq!(5, total_distance_from(1, 0, &coords));
        assert_eq!(6, total_distance_from(1, 1, &coords));
        assert_eq!(8, total_distance_from(0, 2, &coords));

    }

    #[test]
    fn map_over_vector() {
        let result : Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x*2).collect();
        assert_eq!(vec![2, 4, 6], result);
    }

    #[test]
    fn filter_and_map_vector() {
        let v = vec![(5, "coord1"), (3, "coord2"), (3, "coord3")];
        let min = 3;
        let result : Vec<&str> =v.into_iter()
            .filter(|(x, _)| x<=&min)
            .map(|(_, s)| s)
            .collect();
        assert_eq!(vec!["coord2", "coord3"], result);
    }

    #[test]
    fn hashmap_with_tuple_key() {
        let mut hm : HashMap<(i32, i32), i32> = HashMap::new();
        hm.insert((1, 2), 12);
        hm.insert((3, 4), 34);
        assert_eq!(12, *hm.get(&(1, 2)).unwrap());
        assert_eq!(34, *hm.get(&(3, 4)).unwrap());
    }
}










