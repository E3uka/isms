use std::fmt;

const MIN_PER_DAY: i32 = 24 * 60;
const MIN_PER_HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    cyclic_mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            cyclic_mins: Clock::format_time(
                MIN_PER_DAY + Clock::format_time(MIN_PER_HOUR * hours + minutes),
            ),
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
