#![allow(dead_code, unused_variables)]

mod clock {
    /*
    A clock can be initialized or unitialized. You know that from when
    there is a short power outage and the clock is reset to 00:00,
    constantly blinking.
    */

    use std::{fmt::Display, str::FromStr};

    pub enum WallClock {
        Error,
        Uninitialized(WallClockState),
        Initialized(WallClockState),
    }

    impl PartialEq for WallClock {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (WallClock::Error, WallClock::Error) => false,
                (WallClock::Uninitialized(state1), WallClock::Uninitialized(state2)) => {
                    state1 == state2
                },
                (WallClock::Initialized(state1), WallClock::Initialized(state2)) => {
                    state1 == state2
                },
                _ => false,
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct WallClockState {
        hours: u8,
        minutes: u8,
    }

    impl FromStr for WallClockState {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid time format: {}", s));
            }

            let hours: u8 = parts[0].parse().map_err(|_| format!("Invalid hours: {}", parts[0]))?;
            let minutes: u8 = parts[1].parse().map_err(|_| format!("Invalid minutes: {}", parts[1]))?;

            if hours >= 24 || minutes >= 60 {
                return Err(format!("Invalid time value: {}", s));
            }

            Ok(WallClockState { hours, minutes })
        }
    }

    impl Display for WallClockState {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:02}:{:02}", self.hours, self.minutes)
        }
    }

    impl Default for WallClockState {
        fn default() -> Self {
            Self::new()
        }
    }

    impl WallClockState {
        pub fn new() -> Self {
            Self::new_from_time(12, 0)
        }

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

        pub fn get_hours(&self) -> u8 {
            self.hours
        }

        pub fn get_minutes(&self) -> u8 {
            self.minutes
        }

        pub fn add_minutes(&self, minutes: u8) -> Self {
            let total_minutes = self.hours as u16 * 60 + self.minutes as u16 + minutes as u16;
            Self::new_from_minutes(total_minutes)
        }
    }
}

use clock::WallClock;

fn main() {
    let mut my_clock = WallClock::Error;
    print_clock(&my_clock);

    // Something magically happens and the clock is recovering, but it is still uninitialized.
    //my_clock = WallClock::Uninitialized(Default::default());
    my_clock = WallClock::Uninitialized("12:30".parse().unwrap());
    print_clock(&my_clock);

    // A human is setting the clock. We copy the clock time from the uninitialized state to the initialized state.
    /*
    match my_clock {
        WallClock::Uninitialized(state) => {
            my_clock = WallClock::Initialized(state);
        }
        _ => { /* Print some error */}
    }
    */
    if let WallClock::Uninitialized(state) = my_clock {
        my_clock = WallClock::Initialized(state);
    }

    print_clock(&my_clock);
}

fn print_clock(clock: &WallClock) {
    match clock {
        WallClock::Error => println!("Clock is in error state."),
        WallClock::Uninitialized(state) => println!(
            "Clock is uninitialized (blinking) at {:02}:{:02}.",
            state.get_hours(),
            state.get_minutes()
        ),
        WallClock::Initialized(state) => println!(
            "Clock is initialized at {state:?}."
        ),
    }
}
