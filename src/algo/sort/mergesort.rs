use std::collections::LinkedList;

pub fn mergesort(mut numbers: LinkedList<isize>) -> LinkedList<isize> {
    if numbers.len() < 2 {return numbers;}
    let center = numbers.len() / 2;
    let mut head = numbers.split_off(center);

    head = mergesort(head);
    let mut tail = mergesort(numbers);

    let mut merged: LinkedList<isize> = LinkedList::new();
    loop {
        match (head.len(), tail.len()) {
            (0, _) => {
                merged.append(&mut tail);
                return merged;
            }
            (_, 0) => {
                merged.append(&mut head);
                return merged;
            }
            (_, _) => {
                let num = if head.front() < tail.front() {
                    head.pop_front().unwrap()
                } else {
                    tail.pop_front().unwrap()
                };
                merged.push_back(num);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::mergesort;
    use std::collections::LinkedList;
    #[test]
    fn order() {
        //let mut numbers: list<isize>: [isize; 10] = [8, 1, 5, 2, 3, 9, 4, 7, 6, 0];
        let mut numbers: LinkedList<isize> = LinkedList::new();
        for num in &[8, 1, 5, 2, 3, 9, 4, 7, 6, 0] {
            numbers.push_back(*num);
        }
        // 1, 5, 2, 3, 0, 4, 7, 6, 8, 9
        numbers = mergesort(numbers);

        let mut prev = numbers.pop_front();
        while !numbers.is_empty() {
            let actual = numbers.pop_front();
            assert!(prev < actual);
            prev = actual;
        }
        //println!("{:?}", numbers);
    }
}
