use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // old solution
    // start + Duration::seconds(1_000_000_000)

    // good solution that checks for overflow and panics if it does to prevent undefined behaviour.
    start
        .checked_add_signed(Duration::seconds(1_000_000_000))
        .expect("overflow")
}
