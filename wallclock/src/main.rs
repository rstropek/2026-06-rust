#![allow(dead_code, unused_variables)]

mod clock {
    /*
    A clock can be initialized or unitialized. You know that from when
    there is a short power outage and the clock is reset to 00:00,
    constantly blinking.
    */

    pub enum WallClock {
        Error,
        Uninitialized(WallClockState),
        Initialized(WallClockState),
    }

    pub struct WallClockState {
        hours: u8,
        minutes: u8,
    }

    impl WallClockState {
        pub fn new() -> Self {
            Self::new_from_time(0, 0)
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
    my_clock = WallClock::Uninitialized(clock::WallClockState::new_from_time(12, 34));
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
            "Clock is initialized at {:02}:{:02}.",
            state.get_hours(),
            state.get_minutes()
        ),
    }
}
