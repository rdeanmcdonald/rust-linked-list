use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        let node = Node {
            elem,
            next: self.head.as_ref().map(|n| n.clone()),
        };

        List {
            head: Some(Rc::new(node)),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self
                .head
                .as_ref()
                .and_then(|link| link.next.as_ref().map(|n| n.clone())),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|link| &link.elem)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
