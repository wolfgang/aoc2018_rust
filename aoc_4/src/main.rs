extern crate regex;
use regex::Regex;


/* Part 1:
- Create map day -> guard id
- Create map day -> (guard id -> sleep/wake entries )
- Parse above map to generate total minutes asleep
    - sort sleep/awake entries by minute
    - figure out minutes asleep per day
    - sum up per guard
*/

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_day_from_entry() {
        let entry1 = "[1518-10-25 00:29]";
        let re = Regex::new(r"^\[(\d{4}\-\d{2}\-\d{2})").unwrap();
        let caps = re.captures(entry1).unwrap();
        let day = caps.get(1).unwrap().as_str();
        assert_eq!("1518-10-25", day);

    }
}