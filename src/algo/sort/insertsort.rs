pub fn insertsort(numbers: &mut[isize]) {
    for j in 0 .. numbers.len() {
        for i in (0 .. j).rev() {
            if numbers[i] > numbers[i+1] {
                numbers.swap(i, i+1);
            } else {break;}
        }
    }
}

// recursive version
pub fn r_insertsort(numbers: &mut[isize]) {
    let n = numbers.len();

    if n == 1 {return;}
    r_insertsort(&mut numbers[.. n-1]);

    for i in (1 .. n).rev() {
        if numbers[i] < numbers[i-1] {
            numbers.swap(i, i-1);
        } else {break;}
    }

}

#[cfg(test)]
mod test {
    use super::insertsort;
    use super::r_insertsort;
    use super::super::test_common::test_order;
    #[test]
    fn basic() {
        test_order(insertsort);
        test_order(r_insertsort);
    }
}
