
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use std::collections::HashSet;


fn main() -> io::Result<()> {

    let mut offsets = vec![];

    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
    	let line = line.unwrap();
    	let i = line.parse::<i32>().unwrap();
    	offsets.push(i);
    }

    let mut sum = 0;
	let mut freqs = HashSet::new();
	let mut dup_found = false;
	let mut iterations = 0;

	while !dup_found {
		iterations += 1;
	    for i in &offsets {
	        sum += i;
	        if freqs.contains(&sum) {
	        	dup_found = true;
	        	println!("First dup: {} Iterations: {}", sum, iterations);
	        	break;
	        }
	        freqs.insert(sum);	        
	    }

	}

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_assert() {
        assert_eq!(1, 1);
    }
}
