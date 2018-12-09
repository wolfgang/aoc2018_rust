#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

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

struct AreaBuilder {
    width: u32,
    height: u32,
    areas: Vec<Area>
}

impl AreaBuilder {
    fn new(width: u32, height: u32) -> AreaBuilder {
        AreaBuilder {
            width: width, 
            height: height,
            areas: Vec::new()
        }
    }

    fn build(&self, coords: &Vec<Coord>) -> Vec<Area> {
        vec![Area::new(coords[0])]
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

    #[test]
    fn add_coords_to_area() {
        let mut area = Area::new((3, 4));
        area.add_coordinate((5, 6));
        area.add_coordinate((7, 8));
        assert_eq!(vec![(3, 4), (5, 6), (7, 8)], area.coordinates);
    }

    #[test]
    fn initialize_area_builder() {
        let area_builder = AreaBuilder::new(100, 200);
        assert_eq!(100, area_builder.width);
        assert_eq!(200, area_builder.height);
        assert_eq!(Vec::<Area>::new(), area_builder.areas);
    }

    #[test]
    fn build_area_for_one_coord() {
        let area_builder = AreaBuilder::new(1, 1);
        let areas = area_builder.build(&vec![(0, 0)]);
        assert_eq!(1, areas.len());
        assert_eq!(1, areas[0].coordinates.len());
        assert_eq!((0, 0), areas[0].coordinates[0]);

    }


}