pub fn tournament(numbers: &mut[isize]) {
    let n = numbers.len();

    let mut results = vec![0; n];
    for i in 0 .. n - 1 {
        for j in i+1 .. n {
            if numbers[i] > numbers[j] {
                results[i] += 1;
            } else {
                results[j] += 1;
            }
        }
    }

    let mut ordered = vec![0; n];
    for i in 0 .. numbers.len() {
        ordered[results[i]] = numbers[i];
    }

    numbers.copy_from_slice(&ordered);
}

#[cfg(test)]
mod test {
    use super::tournament;
    use super::super::test_common::test_order;
    #[test]
    fn basic() {
        test_order(tournament);
    }
}
