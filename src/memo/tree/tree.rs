use super::iterator::IterTree;

use std::mem;

fn successor(mut next: &mut Link) -> &mut Link {
    loop {
        match { next } {
            &mut Some(ref mut node) if node.left.is_some() => next = &mut node.left,
            other @ &mut Some(_) => return other,
            _ => unreachable!(),
        }
    }
}

pub type Link = Option<Box<Node>>;

pub struct Tree {
    head: Link,
}

pub struct Node {
    pub value: i32,
    pub left: Link,
    pub right: Link,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { head: None }
    }

    pub fn insert(&mut self, elem: i32) {
        Tree::_insert(&mut self.head, elem);
    }

    fn _insert(target: &mut Link, elem: i32) {
        match *target {
            None => {
                *target = Some(Box::new(Node::new(elem)));
            }
            Some(ref mut node) => {
                let target = if elem < node.value {
                    &mut node.left
                } else {
                    &mut node.right
                };
                Tree::_insert(target, elem);
            }
        }
    }

    pub fn search(&self, elem: i32) -> bool {
        let link = Tree::_search(&self.head, elem);
        match *link {
            None => false,
            Some(_) => true,
        }
    }

    fn _search(target: &Link, elem: i32) -> &Link {
        match *target {
            None => target,
            Some(ref node) => {
                if node.value == elem {return target;}
                let target = if elem < node.value {&node.left} else {&node.right};
                Tree::_search(target, elem)
            }
        }
    }

    fn _msearch(target: &mut Link, elem: i32) -> &mut Link {
        let mut anchor = target;
        loop {
            match {anchor}  {
                &mut Some(ref mut node) if elem != node.value => {
                    anchor = if elem < node.value { &mut node.left } else { &mut node.right }
                },
                other => return other
            }
        }
    }

    pub fn delete(&mut self, elem: i32) {
        let target = Tree::_msearch(&mut self.head, elem);
        Tree::_delete(target);
    }

    fn _delete(link: &mut Link) {
        if let Some(mut boxed_node) = link.take() {
            match (boxed_node.left.take(), boxed_node.right.take()) {
                (None, None) => (),
                (Some(left), None) => *link = Some(left),
                (None, Some(right)) => *link = Some(right),
                (Some(left), Some(right)) => {
                    // take() followed by re-assignment looks like an awful hackjob, but appears
                    // to be the only way to satisfy all cases in the match
                    {
                        let node = &mut *boxed_node; // unbox
                        node.left = Some(left);
                        node.right = Some(right);
                        let next = successor(&mut node.right);
                        mem::swap(&mut node.value, &mut next.as_mut().unwrap().value);
                        Tree::_delete(next);
                    }
                    *link = Some(boxed_node);
                }
            }
        }
    }

    pub fn iter(&self) -> IterTree {
        IterTree::new(&self.head)
    }
}

#[cfg(test)]
mod test {
    use super::Tree;
    #[test]
    fn basics() {
        let mut tree = Tree::new();

        assert!(!tree.search(3));

        for entry in &[5, 10, 15, 7, 8, 21, 12, 11, 13] {
            tree.insert(*entry);
        }

        tree.delete(10);

        for entry in &[5, 15, 7, 8, 21, 12, 11, 13] {
            println!("found {:?}", *entry);
            assert!(tree.search(*entry));
        }
    }
}
