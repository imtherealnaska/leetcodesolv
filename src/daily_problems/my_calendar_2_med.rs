struct MyClendarTwo {
    bookings: Vec<(i32, i32)>,
    overlaps: Vec<(i32, i32)>,
}

impl MyClendarTwo {
    fn new() -> Self {
        Self {
            bookings: Vec::new(),
            overlaps: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(o_start, o_end) in &self.overlaps {
            if start < o_end && end > o_start {
                return false;
            }
        }

        for &(b_start, b_end) in &self.bookings {
            let overlap_start = start.max(b_start);
            let overlap_end = end.min(b_end);

            if overlap_start < overlap_end {
                self.overlaps.push((overlap_start, overlap_end));
            }
        }

        self.bookings.push((start, end));
        true
    }
}

#[test]
fn overlap_runs() {}
