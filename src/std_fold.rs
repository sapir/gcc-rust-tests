pub fn test_fold_start_value() -> i32 {
    (0..9).fold(20, |a, _b| a)
}

pub fn test_fold_end_value() -> i32 {
    (0..9).fold(20, |_a, b| b)
}

pub fn test_sum_by_fold() -> i32 {
    (0..9).fold(0, |a, b| a + b)
}
