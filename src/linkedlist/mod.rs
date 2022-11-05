use std::collections::LinkedList;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = std::mem::take(&mut self.head);
        while let Some(mut node) = cur {
            cur = std::mem::take(&mut node.next);
        }
    }
}
impl<T> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { todo!() }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List::<T>{head: None}
    }
    

    pub fn pop_front(&mut self) -> Option<T> {
        let taken = std::mem::take(&mut self.head);
        match taken {
            None => {
                None
            }
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {   
        let n = Some(Box::from(Node::<T>{elem: elem, next: None}));
        if let None = self.head {
            return self.head = n;
        }

        let mut cur = &mut self.head;
        while let Some(node) = cur {
            match &mut node.next {
                None => {return node.next = n;}
                Some(_) => {cur = &mut node.next;}
            }  
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let head = std::mem::take(&mut self.head);
        let n = Some(Box::from(Node::<T>{elem: elem, next: head}));
        self.head = n;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut cur = &mut self.head;
        let mut next: &mut Link<T>;
        // Find me the node where the one after it is None
        match cur {
            None => {return None;}
            Some(node) => {
                next = &mut node.next;
            }
        }
        while let Some(next_node) = next {
            match &mut next_node.next {
                None => {
                    std::mem::take(next);
                }
                Some(node_after) => {
                    let new_next = &mut node_after.next;

                    cur = next;
                    next = new_next;
                    
                }
            }  
        }
        None

    }
}


#[cfg(test)]
mod tests {
    use crate::linkedlist::List;
    #[test]
    fn can_push_front_and_pop_back() {
        let mut l = List::<f64>::new();
        l.push_back(1.0);
        l.push_back(2.0);
        l.push_back(3.0);
        assert_eq!(Some(1.0), l.pop_front());
        assert_eq!(Some(2.0), l.pop_front());
        assert_eq!(Some(3.0), l.pop_front());
    }
    #[test]
    fn can_push_front_and_pop_front() {
        let mut l = List::<f64>::new();
        l.push_front(1.0);
        l.push_front(2.0);
        l.push_front(3.0);
        assert_eq!(Some(3.0), l.pop_front());
        assert_eq!(Some(2.0), l.pop_front());
        assert_eq!(Some(1.0), l.pop_front());
    }
}