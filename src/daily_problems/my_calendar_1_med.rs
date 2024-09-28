use std::{cell::RefCell, collections::BTreeSet};

struct MyCalendar {
    events: RefCell<BTreeSet<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            events: RefCell::new(BTreeSet::new()),
        }
    }

    fn book(&self, start: i32, end: i32) -> bool {
        if start >= end {
            return false;
        }

        let mut events = self.events.borrow_mut();

        if let Some(&(_, prev_end)) = events.range(..=(start, i32::MAX)).next_back() {
            if prev_end > start {
                return false;
            }
        }

        if let Some(&(next_start, _)) = events.range((start, i32::MIN)..).next() {
            if next_start < start {
                return false;
            }
        }

        events.insert((start, end));
        true
    }
}
