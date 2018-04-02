use super::tree::{Link, Node};

pub struct IterTree<'a> {
    stack: Vec<&'a Node>,
    node: &'a Link
}

impl<'a> IterTree<'a>{
    pub fn new(head: &'a Link) -> Self{
        Self {
            stack: vec![],
            node: head,
        }
    }
}

impl<'a> Iterator for IterTree<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ref nd) = *self.node {
            self.stack.push(nd);
            self.node = &nd.left;
        }
        self.stack.pop().map(|nd| {
            self.node = &nd.right;
            nd.value
        })
    }
}
