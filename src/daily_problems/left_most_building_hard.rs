use std::{cmp::Reverse, collections::BinaryHeap};

pub fn left_most_building(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = heights.len();
    let mut ans = vec![-1; queries.len()];

    for (q_idx, query) in queries.iter().enumerate() {
        let mut a = query[0] as usize;
        let mut b = query[1] as usize;

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        if a == b || heights[a] < heights[b] {
            ans[q_idx] = b as i32;
            continue;
        }

        let mut found = false;
        let max_height = heights[a].max(heights[b]);

        for j in (b + 1)..n {
            if heights[j] > max_height {
                ans[q_idx] = j as i32;
                found = true;
                break;
            }
        }

        if !found {
            ans[q_idx] = -1;
        }
    }
    ans
}

fn more_optimised(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut results = vec![-1; queries.len()];
    let mut store_queries: Vec<Vec<(i32, usize)>> = vec![Vec::new(); heights.len()];

    for (idx, query) in queries.iter().enumerate() {
        let (a, b) = (query[0] as usize, query[1] as usize);

        if a < b && heights[a] < heights[b] {
            results[idx] = b as i32;
        } else if a > b && heights[a] > heights[b] {
            results[idx] = a as i32;
        } else if a == b {
            results[idx] = a as i32;
        } else {
            store_queries[a.max(b)].push((heights[a].max(heights[b]), idx));
        }
    }

    let mut max_idx: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

    for (idx, &height) in heights.iter().enumerate() {
        while let Some(Reverse((h, q_idx))) = max_idx.peek() {
            if *h < height {
                results[*q_idx] = idx as i32;
                max_idx.pop();
            } else {
                break;
            }
        }
        for &(h, q_idx) in store_queries[idx].iter() {
            max_idx.push(Reverse((h, q_idx)));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn not_opt() {
        let heights = vec![6, 4, 8, 5, 2, 7];
        let queries = vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 3]];

        let expected = vec![2, 5, 5, 5, 2];
        assert_eq!(left_most_building(heights, queries), expected);
    }

    #[test]
    fn test_no_meeting_point() {
        let heights = vec![5, 4, 3, 2, 1];
        let queries = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let expected = vec![-1, -1, -1];
        assert_eq!(left_most_building(heights, queries), expected);
    }
}
