extern crate regex;
use regex::Regex;

use std::collections::HashMap;

// TODO
// For each line of sorted input:
//  If guard goes on duty, create new GuardRecord
//  Record minutes asleep for current guard
// Find guard record with most sleep minutes
// Produce result -> guard id * minute most asleep 

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
        let mut guard_records : HashMap<i32, GuardRecord> = HashMap::new();
        let mut current_guard_id = -1;
        let mut last_sleep_minute = -1;

        for line in self.input {
            let guard_id = parse_guard_from_entry(line);
            if guard_id != -1 {
                current_guard_id = guard_id;
                if !guard_records.contains_key(&guard_id) {
                    guard_records.insert(guard_id, GuardRecord::new(guard_id));
                }
            }

            let current_guard_record = guard_records.get_mut(&current_guard_id).unwrap();

            if is_sleep_entry(line) {
                last_sleep_minute = parse_minutes_from_entry(line);
            }

            if is_wakeup_entry(line) {
                let wakeup_minute = parse_minutes_from_entry(line);
                current_guard_record.was_asleep(last_sleep_minute, wakeup_minute);
            }
        }

        let mut sleepiest_guard = GuardRecord::new(-1);
        for (_id, guard_record) in guard_records {
            if guard_record.minutes_asleep() > sleepiest_guard.minutes_asleep() {
                sleepiest_guard = guard_record;
            }
        }

        return sleepiest_guard.id * sleepiest_guard.minute_most_asleep();
    }
}

pub struct GuardRecord {
    id: i32,
    sleep_per_minute: HashMap<i32, i32>,
}

impl GuardRecord {
    pub fn new(id: i32) -> GuardRecord {
        return GuardRecord { 
            id: id, 
            sleep_per_minute: HashMap::new()
         };
    }

    pub fn was_asleep(&mut self, from_minute: i32, to_minute: i32) {
        for m in from_minute .. to_minute {
            self.increase_sleep_for_minute(m)
        }
    }

    pub fn minutes_asleep(&self) -> i32 {
        let mut sum = 0;
        for (_, sleep) in &self.sleep_per_minute {
            sum += sleep;
        }

        return sum;
    }

    pub fn minute_most_asleep(&self) -> i32 {
        let mut max_entry = (-1, -1);

        for (minute, sleep) in &self.sleep_per_minute {
            if *sleep > max_entry.1 {
                max_entry = (*minute, *sleep);
            }
        }
        return max_entry.0;
    }

    fn increase_sleep_for_minute(&mut self, minute: i32) {
        if !self.sleep_per_minute.contains_key(&minute) {
            self.sleep_per_minute.insert(minute, 0);
        }

        let count = self.sleep_per_minute.get_mut(&minute).unwrap();
        *count += 1;
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

fn is_sleep_entry(entry: &String) -> bool {
  Regex::new(r"falls asleep$").unwrap().is_match(entry)
}

fn is_wakeup_entry(entry: &String) -> bool {
  Regex::new(r"wakes up$").unwrap().is_match(entry)
}

fn parse_minutes_from_entry(entry: &String) -> i32 {
    let re = Regex::new(r"\d{2}:(\d{2})").unwrap();
    let caps = re.captures(entry).unwrap();
    return caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
}

 
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod tests {
    use super::*;

    #[test]
    fn find_sleepiest_guard() {
        let mut input = part1_input();
        input.sort();

        let gc = GuardFinder::new(&input);
        assert_eq!(10*24, gc.sleepiest_guard());
    }

    #[test]
    fn guard_record_after_initialization() {
        let gr1 = GuardRecord::new(1234);
        assert_eq!(1234, gr1.id);
        assert_eq!(0, gr1.minutes_asleep());
        assert_eq!(-1, gr1.minute_most_asleep());
    }

    #[test]
    fn guard_accumulate_sleep_minutes() {
        let mut gr = GuardRecord::new(1234);
        gr.was_asleep(4, 7);
        assert_eq!(3, gr.minutes_asleep());
        gr.was_asleep(3, 10);
        assert_eq!(10, gr.minutes_asleep());

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
    fn guard_10_sleep_pattern() {
        let mut gr = GuardRecord::new(10);
        gr.was_asleep(5, 25);
        gr.was_asleep(30, 55);
        gr.was_asleep(24, 29);
        assert_eq!(24, gr.minute_most_asleep());
    }

    #[test]
    fn parse_guard_from_entry_() {
        assert_eq!(-1, parse_guard_from_entry(&String::from("[1518-10-25 00:29] falls asleep")));
        assert_eq!(99, parse_guard_from_entry(&String::from("[1518-10-25 00:29] Guard #99 begins shift")));
        assert_eq!(1299, parse_guard_from_entry(&String::from("[1518-10-25 00:29] Guard #1299 begins shift")));
    }

    #[test]
    fn is_sleep_entry_() {
        assert!(is_sleep_entry(&String::from("[1518-10-25 00:29] falls asleep")));
        assert!(!is_sleep_entry(&String::from("[1518-10-25 00:29] Guard #99 begins shift")));
        assert!(!is_sleep_entry(&String::from("[1518-10-25 00:29] wakes up")));
    }

    #[test]
    fn is_wakeup_entry_() {
        assert!(is_wakeup_entry(&String::from("[1518-10-25 00:29] wakes up")));
        assert!(!is_wakeup_entry(&String::from("[1518-10-25 00:29] falls asleep")));
        assert!(!is_wakeup_entry(&String::from("[1518-10-25 00:29] Guard #99 begins shift")));
    }

    #[test]
    fn parse_minutes_from_entry_() {
        assert_eq!(29, parse_minutes_from_entry(&String::from("[1518-10-25 00:29] falls asleep")));
        assert_eq!(5, parse_minutes_from_entry(&String::from("[1518-10-26 00:05] wakses up")));
    }


    #[test]
    fn can_sort_input_by_just_comparing_strings() {
        let mut input = part1_input();
        input.sort();
        assert_eq!(part1_input_chronological(), input);
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