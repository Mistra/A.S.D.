use std::cmp::PartialOrd;

pub fn selectionsort<T: PartialOrd>(numbers: &mut[T]) {
    if numbers.is_empty() {return;}
    let mut position = 0;
    for (i, element) in numbers.iter().enumerate() {
        if *element < numbers[position] {
            position = i;
        }
    }

    numbers.swap(0, position);
    selectionsort(&mut numbers[1 ..]);
}

#[cfg(test)]
mod test {
    use super::selectionsort;
    use super::super::test_common::test_order;
    #[test]
    fn basic() {
        test_order(selectionsort);
    }
}
