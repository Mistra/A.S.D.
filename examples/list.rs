extern crate asd;
use asd::memo::List;

fn main() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    list.pop();

    for val in list.iter() {
        println!("{:?}", val);
    }
}
