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

pub struct GuardFinder<'a> {
    input : &'a Vec<String>

}

impl<'a> GuardFinder<'a> {
    pub fn new(input: &'a Vec<String>) -> GuardFinder<'a> {
        return GuardFinder { input: input};
    }

    pub fn sleepiest_guard(&self) -> i32 {
        
        return 0;
    }
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

    #[test]
    fn part1() {
        let input = vec![
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-01 00:25] wakes up"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up")
        ];

        let gc = GuardFinder::new(&input);
        assert_eq!(10*24, gc.sleepiest_guard());
    }
}