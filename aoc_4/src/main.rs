extern crate regex;
use regex::Regex;
use std::cmp::Ordering;


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

fn sort_chronologically(input: &Vec<String>) -> Vec<String> {    
    return Vec::new();
}

fn parse_day_from_entry(entry : &String) -> String {
    let re = Regex::new(r"^\[(\d{4}\-\d{2}\-\d{2})").unwrap();
    let caps = re.captures(entry).unwrap();
    return caps.get(1).unwrap().as_str().into();
}

fn parse_guard_from_entry(entry: &String) -> i32 {
    let re = Regex::new(r"Guard #(\d+)").unwrap();
    if re.is_match(entry) {
       let caps = re.captures(entry).unwrap();
       return caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    }
    return -1;
}

fn parse_time_values_from_entry(entry: &String) -> (i32, i32, i32, i32) {
    let re = Regex::new(r"^\[\d{4}\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})").unwrap();
    let caps = re.captures(entry).unwrap();
    let month = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let day = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let hour = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let minute = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

    return (month, day, hour, minute);
}

fn compare_entries(entry1: &String, entry2: &String) -> Ordering {
    let (month1, day1, hour1, minute1) = parse_time_values_from_entry(entry1);
    let (month2, day2, hour2, minute2) = parse_time_values_from_entry(entry2);

    let v1 = ((month1 * 30 + day1)*24 + hour1)*60 + minute1;
    let v2 = ((month2 * 30 + day2)*24 + hour2)*60 + minute2;

    return v1.cmp(&v2);
}
 
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod tests {
    use super::*;
    #[test]
    fn parse_day_from_entry_() {
        let entry = String::from("[1518-10-25 00:29] falls asleep");
        let day = parse_day_from_entry(&entry);
        assert_eq!("1518-10-25", day);

    }

    #[test]
    fn parse_time_values_from_entry_() {
        assert_eq!((10, 25, 0, 29), parse_time_values_from_entry(&String::from("[1518-10-25 00:29] Guard #1299 begins shift")));

    }

    #[test]
    fn parse_guard_from_entry_() {
        assert_eq!(-1, parse_guard_from_entry(&String::from("[1518-10-25 00:29] falls asleep")));
        assert_eq!(99, parse_guard_from_entry(&String::from("[1518-10-25 00:29] Guard #99 begins shift")));
        assert_eq!(1299, parse_guard_from_entry(&String::from("[1518-10-25 00:29] Guard #1299 begins shift")));
    }


    #[test]
    fn sort_chronologically_() {
        let mut input = part1_input();
        input.sort();
        // input.sort_by(|a, b| compare_entries(a, b));

        assert_eq!(part1_input_chronological(), input);
    }

    #[test]
    #[ignore]
    fn part1() {
        let input = part1_input();

        let gc = GuardFinder::new(&input);
        assert_eq!(10*24, gc.sleepiest_guard());
    }

    fn part1_input() -> Vec<String> {
        vec![
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
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up")
        ]
    }

    fn part1_input_chronological() -> Vec<String> {
        vec![
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-01 00:25] wakes up"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up")
        ]
    }

}