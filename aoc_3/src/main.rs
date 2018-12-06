fn main() {
    println!("Hello, world!");
}

struct Grid {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<i32>
}

fn grid_new(width: usize, height: usize) -> Grid {
    Grid { width: width, height: height, elements: vec!(0; width*height) }
}

fn grid_set(g: &mut Grid, x: usize, y: usize, value: i32) {
    g.elements[x + y * g.width] = value;

}

fn grid_get(g: &Grid, x: usize, y: usize) -> i32 {
    return g.elements[x + y * g.width];
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid() {
        let mut g = grid_new(10, 20);
        grid_set(&mut g, 5, 6, 1234);
        assert_eq!(1234, grid_get(&g, 5, 6));
    }
}
