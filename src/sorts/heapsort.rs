use std::fmt::Debug;

pub fn heap_sort<T>(arr: &mut [T])
where
    T: Debug + Ord,
{
    let len = arr.len();

    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
        println!("debug stage {i} : heapify {arr:?}");
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }

    println!("debug stage final : heapify {arr:?}");
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_sort_works() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn heapsort_single_element() {
        let mut arr = vec![42];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}
