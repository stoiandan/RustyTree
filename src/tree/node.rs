use std::cell::{Ref, RefCell};
use std::default::Default;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T>
where
    T: Default,
{
    val: T,
    left: Option<RefNode<T>>,
    right: Option<RefNode<T>>,
}

impl<T: Default> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            left: Option::None,
            right: Option::None,
        }
    }

    fn new_ref(val: T) -> Option<RefNode<T>> {
        Some(Rc::new(RefCell::new(Node::new(val))))
    }

    pub fn set_left(&mut self, val: T) {
        self.left = Self::new_ref(val);
    }

    pub fn set_right(&mut self, val: T) {
        self.right = Self::new_ref(val);
    }

    pub fn get_left(&self) -> Option<Ref<T>> {
        self.left
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    pub fn get_right(&self) -> Option<Ref<T>> {
        self.right
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }
}

impl<T> Default for Node<T>
where
    T: Default,
{
    fn default() -> Self {
        Node {
            val: Default::default(),
            right: Option::None,
            left: Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn test_left_and_right() {
        let mut node = Node::new(5);
        node.set_left(3);
        node.set_right(7);
        assert_eq!(*node.get_left().unwrap(), 3);
        assert_eq!(*node.get_right().unwrap(), 7);
    }
}
