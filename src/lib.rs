pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: T) {
        let old_head = self.head.take();

        let new_head = Box::new(Node {
            element,
            next: old_head
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

struct Node<T> {
    element: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::new();

        list.push(2);
        list.push(4);

        list.pop();

        assert_eq!(list.peek(), Some(&2));
    }
}
