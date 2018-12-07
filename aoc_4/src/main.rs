extern crate regex;
use regex::Regex;

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub struct GuardFinder<'a> {
    _input : &'a Vec<String>

}

impl<'a> GuardFinder<'a> {
    pub fn new(input: &'a Vec<String>) -> GuardFinder<'a> {
        return GuardFinder { _input: input};
    }

    pub fn sleepiest_guard(&self) -> i32 {
        // for all input lines 
        // is it a guard? -> Current guard is #xxx, create minutes hashmap for it
        // falling asleep? note current minute
        // waking up ? minutes asleep = minute - sleep minute
        // increase minutes hashmap since last sleep minute
        return 0;
    }
}

pub struct GuardRecord {
    id: i32,
    minutes_asleep: i32,
    sleep_per_minute: HashMap<i32, i32>,
}

impl GuardRecord {
    pub fn new(id: i32) -> GuardRecord {
        return GuardRecord { 
            id: id, 
            minutes_asleep: 0,
            sleep_per_minute: HashMap::new()
         };
    }

    pub fn was_asleep(&mut self, from_minute: i32, to_minute: i32) {
        self.minutes_asleep += to_minute - from_minute;
        for m in from_minute .. to_minute {
            if !self.sleep_per_minute.contains_key(&m) {
                self.sleep_per_minute.insert(m, 0);
            }

            let count = self.sleep_per_minute.get_mut(&m).unwrap();
            *count += 1;
        }
    }

    pub fn minute_most_asleep(&self) -> i32 {
        let mut max_minute = -1;
        let mut max_sleep = -1;

        for (minute, sleep) in &self.sleep_per_minute {
            if *sleep > max_sleep {
                max_sleep = *sleep;
                max_minute = *minute;

            }
        }
        return max_minute;
    }
}

fn parse_guard_from_entry(entry: &String) -> i32 {
    let re = Regex::new(r"Guard #(\d+)").unwrap();
    if re.is_match(entry) {
       let caps = re.captures(entry).unwrap();
       return caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    }
    return -1;
}

 
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod tests {
    use super::*;

    #[test]
    fn guard_record_after_initialization() {
        let gr1 = GuardRecord::new(1234);
        assert_eq!(1234, gr1.id);
        assert_eq!(0, gr1.minutes_asleep);
        assert_eq!(-1, gr1.minute_most_asleep());
    }

    #[test]
    fn guard_accumulate_sleep_minutes() {
        let mut gr = GuardRecord::new(1234);
        gr.was_asleep(4, 7);
        assert_eq!(3, gr.minutes_asleep);
        gr.was_asleep(3, 10);
        assert_eq!(10, gr.minutes_asleep);

    }

    #[test]
    fn guard_record_minute_with_most_sleep() {
        let mut gr = GuardRecord::new(1234);
        gr.was_asleep(10, 11);
        assert_eq!(10, gr.minute_most_asleep());
        gr.was_asleep(10, 12);
        assert_eq!(10, gr.minute_most_asleep());
        gr.was_asleep(11, 14);
        gr.was_asleep(11, 14);
        assert_eq!(11, gr.minute_most_asleep());
        gr.was_asleep(13, 14);
        gr.was_asleep(13, 14);
        assert_eq!(13, gr.minute_most_asleep());
    }

    #[test]
    fn parse_guard_from_entry_() {
        assert_eq!(-1, parse_guard_from_entry(&String::from("[1518-10-25 00:29] falls asleep")));
        assert_eq!(99, parse_guard_from_entry(&String::from("[1518-10-25 00:29] Guard #99 begins shift")));
        assert_eq!(1299, parse_guard_from_entry(&String::from("[1518-10-25 00:29] Guard #1299 begins shift")));
    }


    #[test]
    fn can_sort_input_by_just_comparing_strings() {
        let mut input = part1_input();
        input.sort();
        assert_eq!(part1_input_chronological(), input);
    }

    #[test]
    #[ignore]
    fn part1() {
        let mut input = part1_input();
        input.sort();

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