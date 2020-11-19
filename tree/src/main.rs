use std::cmp::Ordering;
enum Tree<T: Ord> {
    Node {
        val: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    None,
}

impl<T: Ord> Tree<T> {
    fn new() -> Self {
        Tree::None
    }
    fn node(data: T) -> Self {
        Tree::Node {
            val: data,
            left: Box::new(Tree::None),
            right: Box::new(Tree::None),
        }
    }
    fn insert(&mut self, data: T) {
        match self {
            Tree::Node { val, left, right } => match data.cmp(val) {
                Ordering::Less => left.insert(data),
                Ordering::Greater => right.insert(data),
                Ordering::Equal => return,
            },
            Tree::None => {
                *self = Tree::node(data);
            }
        }
    }
    fn find(&mut self, data: T) -> bool {
        match self {
            Tree::Node { val, left, right } => match data.cmp(val) {
                Ordering::Less => left.find(data),
                Ordering::Greater => right.find(data),
                Ordering::Equal => true,
            },
            Tree::None => false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
