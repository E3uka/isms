use chrono::{Duration, NaiveTime};
use std::fmt;

const MIN_PER_DAY: i32 = 24 * 60;
const MIN_PER_HOUR: i32 = 60;

// _secret field is zero sized type this prevent illegal initiliasation & compiles away entirely
// during compiler optimisations, prevents errors like below:
//
// Clock { hours: -24, minutes: -1440 } // hours and minutes should wrap
//
// ref: https://steveklabnik.com/writing/structure-literals-vs-constructors-in-rust
#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    cyclic_mins: i32,
    _secret: (),
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            cyclic_mins: Clock::format_time(
                MIN_PER_DAY + Clock::format_time(MIN_PER_HOUR * hours + minutes),
            ),
            _secret: (),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.cyclic_mins + minutes)
    }

    // returns a representation of cyclic time formatted for minutes in a day.
    fn format_time(input: i32) -> i32 {
        input % MIN_PER_DAY
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.cyclic_mins / MIN_PER_HOUR,
            self.cyclic_mins % MIN_PER_HOUR,
        )
    }
}

// Alternate method using chrono package.
#[derive(Debug, Eq, PartialEq)]
pub struct AlternateClock {
    time: NaiveTime,
    _secret: (),
}

impl AlternateClock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        AlternateClock {
            time: NaiveTime::from_hms(0, 0, 0)
                + Duration::hours(hours as i64)
                + Duration::minutes(minutes as i64),
            _secret: (),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        AlternateClock {
            time: self.time + Duration::minutes(minutes as i64),
            _secret: (),
        }
    }
}

impl fmt::Display for AlternateClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.format("%H:%M").to_string())
    }
}
