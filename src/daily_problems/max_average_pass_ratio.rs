use std::collections::BinaryHeap;

pub fn max_average_pass_ratio(mut classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let mut heap = BinaryHeap::new();
    let calculate_gain = |passes: i32, total: i32| -> f64 {
        (passes as f64 + 1.0) / (total as f64 + 1.0) - (passes as f64 / total as f64)
    };

    for class in classes.iter() {
        let gain = calculate_gain(class[0], class[1]);
        heap.push((gain.to_bits(), class[0], class[1]));
    }

    for _ in 0..extra_students {
        let (_, passes, total) = heap.pop().unwrap();
        let new_passes = passes + 1;
        let new_total = total + 1;
        let new_gain = calculate_gain(new_passes, new_total);
        heap.push((new_gain.to_bits(), new_passes, new_total));
    }

    let mut total_ratio = 0.0;
    while let Some((_, passes, total)) = heap.pop() {
        total_ratio += passes as f64 / total as f64;
    }

    total_ratio / classes.len() as f64
}
