extern crate alloc;

mod contract;
mod mimc;

pub fn slow_fibonacci(nth: usize) -> u64 {
    if nth <= 1 {
        return nth as u64;
    } else {
        return slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2);
    }
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..nth {
        b = a + b;
        a = b - a;
    }
    b
}