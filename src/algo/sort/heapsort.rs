use std::cmp::PartialOrd;

fn heapify<T: PartialOrd>(numbers: &mut [T], n: usize, i: usize) {
	let mut largest = i; // Initialize largest as root
	let l = (i << 1) + 1; // left = 2*i + 1
	let r = l + 1; // right = 2*i + 2

	if l < n && numbers[l] > numbers[largest] {
		largest = l;
	}

	if r < n && numbers[r] > numbers[largest] {
		largest = r;
	}

	if largest != i {
		numbers.swap(i, largest);
		heapify(numbers, n, largest);
	}
}

pub fn heapsort<T: PartialOrd>(numbers: &mut [T]) {
	let n = numbers.len();

	for i in (0..n >> 1).rev() {
		heapify(numbers, n, i);
	}

	for i in (0..numbers.len()).rev() {
		numbers.swap(0, i);
		heapify(numbers, i, 0);
	}
}

#[cfg(test)]
mod test {
    use super::heapsort;
    use super::super::test_common::test_order;
    #[test]
    fn basic() {
        test_order(heapsort);
    }
}
