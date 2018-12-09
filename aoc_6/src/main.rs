#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

type Coord = (i32, i32);

fn main() {
    println!("Hello, world!");
}

fn md(c1 : Coord, c2: Coord) -> u32 {
    let (x1, y1) = c1;
    let (x2, y2) = c2;

    return ((x2 - x1).abs() + (y2 - y1).abs()) as u32;
}

struct Area {
    coordinates: Vec<Coord>
}

impl Area {
    fn new(initial_coord: Coord) -> Area {
        Area {coordinates: vec![initial_coord]}
    }
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
}