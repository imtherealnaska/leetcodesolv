pub fn find_length_shortest_subarray(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut left = 0;
    let mut right = n - 1;

    while left + 1 < n && arr[left] <= arr[left + 1] {
        left += 1;
    }

    if left == n - 1 {
        return 0;
    }

    while right > 0 && arr[right - 1] <= arr[right] {
        right -= 1;
    }

    let mut min_to_remove = (right as i32).min(n as i32 - 1 - left as i32);

    let mut i = 0;
    let mut j = right;

    while i <= left && j < n {
        if arr[i] <= arr[j] {
            min_to_remove = min_to_remove.min(j as i32 - i as i32 - 1);
            i += 1;
        } else {
            j += 1;
        }
    }
    min_to_remove
}
