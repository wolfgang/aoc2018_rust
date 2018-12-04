
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut sum = 0;
    for line in f.lines() {
    	let line = line.unwrap();
        //println!("{}", line);
        let i = line.parse::<i32>().unwrap();
        //println!("{}", i);
        sum += i;
    }

    println!("{}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_assert() {
        assert_eq!(1, 1);
    }
}
