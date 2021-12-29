use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECOND: i64 = 1_000_000_000;
    const DURATION: Duration = Duration::seconds(GIGASECOND);
    start.checked_add(DURATION).unwrap()
}
