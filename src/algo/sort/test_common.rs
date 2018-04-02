#[cfg(test)]
pub fn test_order(sort: fn(&mut [isize])) {
    let mut numbers: [isize; 10] = [8, 1, 5, 2, 3, 9, 4, 7, 6, 0];
    sort(&mut numbers);

    for i in 1 .. numbers.len() {
        assert!(numbers[i-1] < numbers[i]);
    }
}
