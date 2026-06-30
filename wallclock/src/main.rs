#![allow(dead_code, unused_variables)]

mod clock {
    /// A wall clock that stores normalized hours and minutes.
    pub struct WallClock {
        hours: u8,
        minutes: u8,
    }

    /// Builder for creating a [`WallClock`] from hour and minute components.
    pub struct WallClockBuilder {
        hours: u8,
        minutes: u8,
    }

    impl WallClock {
        /// Creates a clock initialized to `00:00`.
        pub fn new() -> Self {
            Self::new_from_time(0, 0)
        }

        /// Creates a builder initialized to `00:00`.
        pub fn builder() -> WallClockBuilder {
            WallClockBuilder::new()
        }

        /// Creates a clock from hour and minute components, normalizing overflow.
        pub fn new_from_time(hours: u8, minutes: u8) -> Self {
            let total_minutes = hours as u16 * 60 + minutes as u16;
            Self::new_from_minutes(total_minutes)
        }

        fn new_from_minutes(total_minutes: u16) -> Self {
            Self {
                hours: ((total_minutes / 60) % 24) as u8,
                minutes: (total_minutes % 60) as u8,
            }
        }

        /// Returns the normalized hour component.
        pub fn get_hours(&self) -> u8 {
            self.hours
        }

        /// Returns the normalized minute component.
        pub fn get_minutes(&self) -> u8 {
            self.minutes
        }
    
        /// Returns a new clock with the given number of minutes added.
        pub fn add_minutes(&self, minutes: u8) -> Self {
            let total_minutes = self.hours as u16 * 60 + self.minutes as u16 + minutes as u16;
            Self::new_from_minutes(total_minutes)
        }
    }

    impl WallClockBuilder {
        fn new() -> Self {
            Self {
                hours: 0,
                minutes: 0,
            }
        }

        /// Sets the hour component.
        pub fn hours(mut self, hours: u8) -> Self {
            self.hours = hours;
            self
        }

        /// Sets the minute component.
        pub fn minutes(mut self, minutes: u8) -> Self {
            self.minutes = minutes;
            self
        }

        /// Builds a normalized [`WallClock`].
        pub fn build(self) -> WallClock {
            WallClock::new_from_time(self.hours, self.minutes)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::WallClock;

        #[test]
        fn new_starts_at_midnight() {
            let clock = WallClock::new();

            assert_eq!(clock.get_hours(), 0);
            assert_eq!(clock.get_minutes(), 0);
        }

        #[test]
        fn new_from_time_normalizes_overflow() {
            let clock = WallClock::new_from_time(25, 90);

            assert_eq!(clock.get_hours(), 2);
            assert_eq!(clock.get_minutes(), 30);
        }

        #[test]
        fn add_minutes_returns_new_normalized_clock() {
            let original = WallClock::new_from_time(23, 30);
            let changed = original.add_minutes(90);

            assert_eq!(original.get_hours(), 23);
            assert_eq!(original.get_minutes(), 30);
            assert_eq!(changed.get_hours(), 1);
            assert_eq!(changed.get_minutes(), 0);
        }

        #[test]
        fn builder_builds_normalized_clock() {
            let clock = WallClock::builder().hours(48).minutes(75).build();

            assert_eq!(clock.get_hours(), 1);
            assert_eq!(clock.get_minutes(), 15);
        }
    }
}

use clock::WallClock;

fn main() {
    let my_clock = WallClock::builder()
        .hours(1)
        .minutes(30)
        .build()
        .add_minutes(150);
    println!("My clock shows: {:02}:{:02}", my_clock.get_hours(), my_clock.get_minutes());
}
