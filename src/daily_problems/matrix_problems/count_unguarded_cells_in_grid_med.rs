use std::collections::HashSet;

// pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {}

pub fn count_unguarded_basic(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let guard_set: HashSet<_> = guards.iter().cloned().collect();
    let wall_set: HashSet<_> = walls.iter().cloned().collect();

    let mut guarded_cells = HashSet::new();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for guard in &guards {
        for (dx, dy) in &directions {
            let mut x = guard[0] + dx;
            let mut y = guard[1] + dy;

            while x >= 0 && x < m && y >= 0 && y < n {
                let cell = vec![x, y];

                if guard_set.contains(&cell) || wall_set.contains(&cell) {
                    break;
                }

                guarded_cells.insert(cell);

                x += dx;
                y += dy;
            }
        }
    }

    let mut unguarded_count = 0;
    for x in 0..m {
        for y in 0..n {
            let cell = vec![x, y];
            if !guard_set.contains(&cell)
                && !wall_set.contains(&cell)
                && !guarded_cells.contains(&cell)
            {
                unguarded_count += 1;
            }
        }
    }
    unguarded_count
}
