// use is_morning::is_morning;

use std::sync::{Arc, Mutex};

use chrono::{offset::LocalResult, prelude::*};

fn main() {
    println!("Hello, world!");
}

trait TimeSource {
    fn now() -> DateTime<Utc>;
}

struct DateTimeSource;

impl TimeSource for DateTimeSource {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

const US_EASTERN_OFFSET: u32 = 5;
const MORNING_BEGINNING_HOUR: u32 = 7; // morning starts at 7am

pub fn is_morning() -> bool {
    let utc = DateTimeSource::now();

    println!("{utc}");

    let morning_beginning: DateTime<Utc> = Utc
        .with_ymd_and_hms(
            utc.year(),
            utc.month(),
            utc.day(),
            MORNING_BEGINNING_HOUR + US_EASTERN_OFFSET,
            0,
            0,
        )
        .unwrap();

    let just_before_morning = morning_beginning - chrono::Duration::nanoseconds(1);

    let just_after_morning = morning_beginning + chrono::Duration::nanoseconds(1);

    utc > just_before_morning && utc < just_after_morning
}

#[cfg(test)]
mod tests {

    use super::is_morning;
    use crate::{TimeSource, MORNING_BEGINNING_HOUR, US_EASTERN_OFFSET};
    use chrono::{DateTime, Datelike, TimeZone, Utc};
    use std::sync::{Arc, Mutex};

    #[test]
    pub fn beginning_of_morning() {
        struct FakeTimeSource(Arc<Mutex<DateTime<Utc>>>);

        impl TimeSource for FakeTimeSource {
            fn now() -> DateTime<Utc> {
                let utc = Utc::now();

                let beginning_of_morning = Utc
                    .with_ymd_and_hms(
                        utc.year(),
                        utc.month(),
                        utc.day(),
                        MORNING_BEGINNING_HOUR + US_EASTERN_OFFSET,
                        0,
                        0,
                    )
                    .unwrap();

                beginning_of_morning
            }
        }

        assert_eq!(is_morning(), true);
    }

    #[test]
    pub fn just_before_morning() {
        assert_eq!(is_morning(), false);
    }
}
