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
}
