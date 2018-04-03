use super::list::Node;

pub struct IterList<'bind> {
    current: &'bind Option<Box<Node>>,
}

impl<'bind> IterList<'bind> {
    pub fn new(node: &'bind Option<Box<Node>>) -> Self {
        Self{ current: node }
    }
}

impl<'bind> Iterator for IterList<'bind> {
    type Item = &'bind i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.as_ref().map(|node| {
            self.current = &node.next;
            &node.elem
        })
    }
}
