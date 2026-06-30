#![allow(dead_code, unused_variables)]

mod clock {
    pub struct WallClock {
        hours: u8,
        minutes: u8,
    }

    impl WallClock {
        pub fn new() -> Self {
            Self::new_from_time(0, 0)
        }

        pub fn new_from_time(hours: u8, minutes: u8) -> Self {
            Self { hours, minutes }
        }

        pub fn get_hours(&self) -> u8 {
            self.hours
        }

        pub fn get_minutes(&self) -> u8 {
            self.minutes
        }
    
        pub fn add_minutes_1(&mut self, minutes: u8) {
            // Variant 1) mutable
            let total_minutes = self.hours as u16 * 60 + self.minutes as u16 + minutes as u16;
            self.hours = ((total_minutes / 60) % 24) as u8;
            self.minutes = (total_minutes % 60) as u8;
        }

        pub fn add_minutes_2(&self, minutes: u8) -> Self {
            // Variant 2) immutable
            let total_minutes = self.hours as u16 * 60 + self.minutes as u16 + minutes as u16;
            let new_hours = ((total_minutes / 60) % 24) as u8;
            let new_minutes = (total_minutes % 60) as u8;
            Self::new_from_time(new_hours, new_minutes)
        }
    }
}

use clock::WallClock;

fn main() {
    let mut my_clock = WallClock::new();
    my_clock.add_minutes_1(150);
    let my_clock = my_clock.add_minutes_2(150);
    println!("My clock shows: {:02}:{:02}", my_clock.get_hours(), my_clock.get_minutes());
}
