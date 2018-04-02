extern crate asd;
use asd::algo::sort::heapsort as sort;

pub fn main() {
    let mut numbers: [isize; 10] = [8, 1, 5, 2, 3, 9, 4, 7, 6, 0];
    sort(&mut numbers);

    for num in numbers.iter() {
        println!("{:?}", num);
    }
}
