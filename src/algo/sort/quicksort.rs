use std::cmp::PartialOrd;
/// Returns a person with the name given them
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person
///
/// # Example
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to Rustdoc, it will even test it for you!
///
/// # extern crate asd;
/// # use asd::algo::sort::quicksort;
/// # fn main() {
/// let mut numbers: [isize; 10] = [8, 1, 5, 2, 3, 9, 4, 7, 6, 0];
/// quicksort(&mut numbers);
///
/// for num in numbers.iter() {
///     println!("{:?}", num);
/// }
/// # }
/// ```
pub fn quicksort<T: PartialOrd>(numbers: &mut[T]) {
    if numbers.len() <= 1 {return;}
    let mut x = 0;
    let mut y = numbers.len() - 1;
    while x != y {
        if numbers[x] > numbers[x+1] {
            numbers.swap(x, x+1);
            x += 1;
        } else {
            numbers.swap(x+1, y);
            y -= 1;
        }
    }
    quicksort(&mut numbers[.. x]);
    quicksort(&mut numbers[x+1 ..]);
}

#[cfg(test)]
mod test {
    use super::quicksort;
    use super::super::test_common::test_order;
    #[test]
    fn basic() {
        test_order(quicksort);
    }
}
