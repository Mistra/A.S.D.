extern crate asd;
use asd::memo::Tree;

fn main() {
    let mut tree = Tree::new();
    for entry in &[5, 10, 15, 7, 8, 21, 12, 11, 13] {
        tree.insert(*entry);
    }

    for var in tree.iter() {
        println!("{:?}", var);
    }
}
