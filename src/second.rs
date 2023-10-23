pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let next = self.head.take();
        let link = Some(Box::new(Node { elem, next }));

        self.head = link;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// ITERATOR
pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn push_pop_i32() {
        let mut list: List<i32> = List::new();

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

    #[test]
    fn push_pop_string() {
        let mut list: List<String> = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(String::from_str("hello").unwrap());
        list.push(String::from_str("world").unwrap());
        list.push(String::from_str("backwards").unwrap());

        // Check normal removal
        assert_eq!(list.pop(), Some(String::from_str("backwards").unwrap()));
        assert_eq!(list.pop(), Some(String::from_str("world").unwrap()));

        // Push some more just to make sure nothing's corrupted
        list.push(String::from_str("again").unwrap());
        list.push(String::from_str("why").unwrap());

        // Check normal removal
        assert_eq!(list.pop(), Some(String::from_str("why").unwrap()));
        assert_eq!(list.pop(), Some(String::from_str("again").unwrap()));

        // Check exhaustion
        assert_eq!(list.pop(), Some(String::from_str("hello").unwrap()));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.peek(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.peek(), Some(&2));
    }

    #[test]
    fn peek_mut() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);

        list.peek_mut().map(|val| {
            *val += 1;
        });

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        let mut iter = list.iter_mut();

        let mut two = iter.next();
        let mut one = iter.next();

        assert_eq!(one, Some(&mut 1));
        assert_eq!(two, Some(&mut 2));
        assert_eq!(iter.next(), None);

        two.as_deref_mut().map(|i| *i -= 1);
        one.as_deref_mut().map(|i| *i -= 1);

        assert_eq!(one, Some(&mut 0));
        assert_eq!(two, Some(&mut 1));
    }
}
