use std::mem;

pub struct List {
    head: Link,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug, Eq, PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
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
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let list = List::new();
        assert_eq!(Link::Empty, list.head)
    }

    #[test]
    fn test_push_and_pop() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        assert_eq!(1, list.pop().unwrap());
        assert_eq!(2, list.pop().unwrap());
        assert_eq!(3, list.pop().unwrap());
        assert!(list.pop().is_none());
    }

    #[test]
    fn basics() {
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

    #[test]
    fn tail_recursion() {
        let mut list = List::new();

        for _n in 1..100000 {
            list.push(1);
        }

        assert!(list.pop().is_some());
    }
}
