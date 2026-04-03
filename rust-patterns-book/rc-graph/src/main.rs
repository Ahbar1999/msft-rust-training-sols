use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct Node {
    pub id: usize,
    // directed edges
    pub forward_edges: Vec<Rc<RefCell<Node>>>,
    pub back_edges: Vec<Weak<RefCell<Node>>>, 
}

impl Node {
    fn new(id: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Node { 
                id,
                forward_edges: Vec::new(),
                back_edges: Vec::new()
        }))
    }

    pub fn add_node(&mut self, id: usize) -> Rc<RefCell<Node>> {
        let new_node = Node::new(id);
        // self.children.push(new_node.clone());
        
        // self. is borrowed as mutable here, why can we borrow self mutable again ????
        self.add_fwdlink(new_node.clone());
        new_node
    } 

    pub fn add_fwdlink(&mut self, other: Rc<RefCell<Node>>) {
        // add a directed edge from self to other
        // self -> other
        self.forward_edges.push(other);
    } 

    pub fn add_backlink(&mut self, other: &Rc<RefCell<Node>>) {
        self.back_edges.push(Rc::downgrade(other));
    }
}

fn main() {
    // println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*; 

    #[test]
    fn basics() {
        // 0[head] -> 1 -> 2 -> 3 -> 0 
        let head = Node::new(0);
        let mut curr = head.clone();
        
        for id in 1..4 {
            let new_node =curr.borrow_mut().add_node(id);
            assert_eq!(Rc::strong_count(&curr), 2);
            curr = new_node; 
        }
        
        curr.borrow_mut().add_backlink(&head.clone());
        drop(curr);
        
        let mut pointer = head.clone();

        for id in 0..4 {
            println!("{:?}", pointer.borrow().id);
            assert_eq!(pointer.borrow().id, id);
            assert_eq!(Rc::strong_count(&pointer), 2);    // pointer + original
            
            if id == 3 {
                // check (weak) back link of last node
                let actual_head= pointer.borrow().back_edges.first().unwrap().upgrade().unwrap();
                assert_eq!(actual_head.borrow().id, 0);
            }

            pointer = {
                if let Some(node_ref) = pointer.borrow().forward_edges.first() {
                    node_ref.clone()
                } else {
                    break;
                }
            };
        }
    }
}


