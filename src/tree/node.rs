mod node {
    
use std::cell::Cell;
use std::rc::Rc;

type NodePtr<T> = Rc<Cell<Node<T>>>;

struct Node<T> {
    value: Cell<T>,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
    parent: Option<NodePtr<T>>
}

}