use chrono::{DateTime, Utc};
use chrono::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let dur = Duration::seconds(10i64.pow(9));
    start + dur
}
