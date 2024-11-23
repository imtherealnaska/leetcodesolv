pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    let mut result = vec![0; n];

    if k == 0 {
        return result;
    }

    let k_abs = k.abs() as usize;
    let mut window_sum = 0;

    if k > 0 {
        for i in 0..k_abs {
            window_sum += code[i + 1];
        }

        for i in 0..n {
            result[i] = window_sum;
            window_sum -= code[(i + 1) % n];
            window_sum += code[(i + k_abs + 1) % n];
        }
    } else {
        for i in 0..k_abs {
            window_sum += code[n - i - 1];
        }

        for i in 0..n {
            result[i] = window_sum;
            window_sum -= code[(n + i - k_abs) % n];
            window_sum += code[i];
        }
    }
    result
}

pub fn decrypt_func(code: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        return vec![0; code.len()];
    }

    let n = code.len();
    let k_abs = k.abs() as usize;
    let circular = code.iter().cycle();

    (0..n)
        .map(|i| {
            if k > 0 {
                circular.clone().skip(i + 1).take(k_abs).sum()
            } else {
                circular.clone().skip(i + n - k_abs).take(k_abs).sum()
            }
        })
        .collect()
}
