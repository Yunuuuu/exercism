use std::fmt;

const DAY_HOUR: isize = 24;
const HOUR: isize = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: isize, 
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes.div_euclid(HOUR).rem_euclid(DAY_HOUR), self.minutes.rem_euclid(HOUR))
    }
}

impl Clock {
	pub fn new(hours: isize, minutes: isize) -> Self {
        let my_hours = (hours + minutes.div_euclid(HOUR)).rem_euclid(DAY_HOUR);
        let my_minutes = minutes.rem_euclid(HOUR);
		Clock {
            minutes: my_hours * HOUR + my_minutes
		}
	}
	pub fn add_minutes(&self, minutes: isize) -> Self {
		Clock {
            minutes: self.minutes + minutes
		}
	}
}
