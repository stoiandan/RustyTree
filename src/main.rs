use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::default::Default;

type RefNode<T> = Rc<RefCell<Node<T>>>;

struct Node<T> where T: Default {
     val: T,
     left: Option<RefNode<T>>,
     right: Option<RefNode<T>>
}


impl<T: Default> Node<T> {
         fn new(val: T) -> Self {
             Node {
                 val,
                 left: Option::None,
                 right: Option::None
             }
         }
         
         fn new_ref(val: T) -> RefNode<T> {
                Rc::new(RefCell::new(Node::new(val)))
         }
         
         
         fn set_left(&mut self, val: T) {
             self.left = Some(Self::new_ref(val));
         }
         
         
         fn set_right(&mut self, val: T) {
             self.right = Some(Self::new_ref(val));
         }
         
         fn get_left(&self) -> Option<Ref<T>> {
               self.left.as_ref().map(|node| Ref::map(node.borrow(), |node| &node.val))
         }
        
}

impl<T> Default for Node<T> where T: Default {
      fn default() -> Self {
          Node {
              val: Default::default(),
              right: Option::None,
              left: Option::None
          }
      }
}


fn main() {
    let mut root = Node::new(4);
    root.set_left(10);
    root.set_right(15);
    
    if let Some(ref val) = root.left {
            println!("{}", val.borrow().val);

    }

    if let Some( val) = root.get_left() {
        println!("{}", val);
    }


    
}