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
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*; 

    #[test]
    fn basics() {
        // 0[head] -> 1 -> 2 -> 3 -> 0 
        let mut head = Node::new(0);
        let mut curr = head.clone();
        
        for id in 1..4 {
            let new_node =curr.borrow_mut().add_node(id);
            assert_eq!(Rc::strong_count(&curr), 2);
            // println!("node with id: {:?} has Rc count = 2", id);
            curr = new_node; 
        }
        
        curr.borrow_mut().add_backlink(&head.clone());
        
        curr = head.clone();    // so head doesn't get dropped when we are done with it
        // drop(curr);

        // 0[head] node
        let mut pointer = head.clone();
        assert_eq!(pointer.borrow().id, 0);
        assert_eq!(Rc::strong_count(&pointer), 3);    // curr + pointer + original(head) + backlink(weak link, so doesnt)
        assert_eq!(Rc::weak_count(&pointer), 1);
        
        for id in 1..4 {
            pointer = {
                if let Some(node_ref) = head.borrow().forward_edges.first() {
                    node_ref.clone()
                } else {
                    break;
                }
            };

            // pointer = head.borrow().forward_edges.first().unwrap().clone();
            assert_eq!(pointer.borrow().id, id);
            assert_eq!(Rc::strong_count(&pointer), 2);    // pointer + original
            head = pointer;

            if id == 3 {
                if let Some(back_link) = head.borrow().back_edges.first() {
                    let actual_head = back_link.upgrade().unwrap(); // this only
                                                                                       // works if
                                                                                       // there
                                                                                       // exists
                                                                                       // atleast
                                                                                       // one
                                                                                       // strong
                                                                                       // reference
                    assert_eq!(actual_head.borrow().id, 0);
                } else {
                    println!("no back link exists! on node with id: {:?}", head.borrow().id);
                }
            }
        }
        
        /*
        // 1 node
        pointer = head.borrow().forward_edges.first().unwrap().clone();
        assert_eq!(pointer.borrow().id, 1);
        assert_eq!(Rc::strong_count(&pointer), 2);    // pointer + original 
        
        head = pointer;
        // 2 node
        pointer = head.borrow().forward_edges.first().unwrap().clone();
        assert_eq!(pointer.borrow().id, 2);
        assert_eq!(Rc::strong_count(&pointer), 2);    // pointer + original 
        
        head = pointer;

        // 3 node
        pointer = head.borrow().forward_edges.first().unwrap().clone();
        assert_eq!(pointer.borrow().id, 3);
        assert_eq!(Rc::strong_count(&pointer), 2);    // pointer + original 
        */

        /*
        // increment count)
        let mut id = 1;
        // last node doesnt have any forward edges
        while let curr_pointer_ref = pointer.borrow() {
            if let Some(curr_pointer) = curr_pointer_ref.forward_edges.first() {
                assert_eq!(curr_pointer.borrow().id, id);
                assert_eq!(Rc::strong_count(curr_pointer), 1);    // original link only
                id += 1;
            } else {
                break;
            }
        }
        */
    }
}


