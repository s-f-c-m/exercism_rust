use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        hours = match hours.cmp(&0) {
            Ordering::Less => {
                while hours < 0 {
                    hours += 24;
                }
                hours
            }
            _ => hours % 24,
        };

        minutes = match minutes.cmp(&0) {
            Ordering::Greater => {
                while minutes > 59 {
                    hours = match hours.cmp(&23) {
                        Ordering::Less => hours + 1,
                        _ => 0,
                    };
                    minutes -= 60;
                }
                minutes
            }

            Ordering::Equal => 0,
            Ordering::Less => {
                while minutes < 0 {
                    hours = match hours.cmp(&0) {
                        Ordering::Equal => 23,
                        _ => hours - 1,
                    };
                    minutes += 60;
                }
                minutes
            }
        };

        Clock {
            hours: hours % 24,
            minutes,
        }
    }

    pub fn add_minutes(mut self, mut minutes: i32) -> Self {
        self.minutes = match minutes.cmp(&0) {
            Ordering::Greater => {
                minutes += self.minutes;
                while minutes > 59 {
                    self.hours = match self.hours.cmp(&23) {
                        Ordering::Less => self.hours + 1,
                        _ => 0,
                    };
                    minutes -= 60;
                }
                minutes
            }
            Ordering::Equal => self.minutes,
            Ordering::Less => {
                minutes = self.minutes + minutes;
                while minutes < 0 {
                    self.hours = match self.hours.cmp(&0) {
                        Ordering::Equal => 23,
                        _ => self.hours - 1,
                    };
                    minutes += 60;
                }
                minutes
            }
        };
        self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let zeroes = |&num: &i32| match num.cmp(&10) {
            Ordering::Less => "0",
            _ => "",
        };
        write!(
            fmt,
            "{}{}:{}{}",
            zeroes(&self.hours),
            self.hours,
            zeroes(&self.minutes),
            self.minutes
        )
    }
}
