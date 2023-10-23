use std::mem;
use std::ptr;

pub struct List {
    head: Link,
}
pub enum Link {
    Empty,
    More(Box<Node>),
}
pub struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn replace(&mut self) -> Link {
        // let head = self.head;
        let head_ptr: *const Link = &self.head;
        let head;
        unsafe {
            head = ptr::read(head_ptr);
        }
        head
        // unsafe {
        //     let result = ptr::read(dest);
        //     ptr::write(dest, src);
        //     result
        // }
    }
    pub fn push(&mut self, elem: i32) {
        let next = mem::replace(&mut self.head, Link::Empty);
        let link = Link::More(Box::new(Node { elem, next }));

        self.head = link;
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_pop() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
