pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }

    (1..=num).fold(1, |acc: u64, n: u64| acc * n)
}
