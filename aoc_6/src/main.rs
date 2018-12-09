#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::collections::HashMap;

type Coord = (i32, i32);

fn main() {
    panic!("No solutions provided");
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
    fn new(initial_coord: Coord) -> Area {
        Area {coordinates: vec![initial_coord]}
    }

    fn add_coordinate(&mut self, coord: Coord) {
        self.coordinates.push(coord);
    }
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

    return all_nearest.into_iter().filter(|(dist, _)| dist==&nearest_distance).map(|(_, coord)| coord).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhatten_distance() {
        assert_eq!(0,  md((0, 0), (0, 0)));
        assert_eq!(31,  md((1, 2), (14, 20)));
        assert_eq!(31,  md((14, 20), (1, 2)));
    }

    #[test]
    fn initialize_area() {
        let area = Area::new((1, 2));
        assert_eq!(vec![(1, 2)], area.coordinates);
    }

    #[test]
    fn add_coords_to_area() {
        let mut area = Area::new((3, 4));
        area.add_coordinate((5, 6));
        area.add_coordinate((7, 8));
        assert_eq!(vec![(3, 4), (5, 6), (7, 8)], area.coordinates);
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










