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

fn rect_from(_s: &String) -> Rect {
    // let rect_part = s.split("@").collect()[0];

    Rect {x: 0, y: 0, width: 0, height: 0}
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
    #[ignore]
    fn parse_rect_from_string() {
        let s1 = String::from("#1200 @ 94,536: 22x13");
        let parsed_rect = rect_from(&s1);

        assert_eq!(94, parsed_rect.x);
        assert_eq!(536, parsed_rect.y);
        assert_eq!(22, parsed_rect.width);
        assert_eq!(13, parsed_rect.height);

    }
}
