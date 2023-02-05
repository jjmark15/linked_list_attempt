extern crate core;

#[derive(Debug, Eq, PartialEq)]
pub struct LinkedList<T> {
    node: Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { node: Node::Empty }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = LinkedList::new();

        vec.into_iter().for_each(|v| list.push(v));

        list
    }

    pub fn to_vec(self) -> Vec<T> {
        todo!()
    }

    pub fn push(&mut self, val: T) {
        self.node.push(val);
    }

    pub fn pop(&mut self) -> T {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.node.size()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Node<T> {
    Empty,
    Tail { value: T },
    Parent { value: T, next: Box<Node<T>> },
}

impl<T> Node<T> {
    fn push(&mut self, val: T) {
        match self {
            Node::Empty => *self = Node::Tail { value: val },
            Node::Tail { .. } => {
                let mut placeholder = Node::Empty;
                std::mem::swap(self, &mut placeholder);

                let extracted_value = placeholder.value();

                *self = Node::Parent {
                    value: extracted_value,
                    next: Box::new(Node::Tail { value: val }),
                };
            }
            Node::Parent { next, .. } => next.push(val),
        };
    }

    fn value(self) -> T {
        match self {
            Node::Empty => panic!("expected value node"),
            Node::Tail { value } => value,
            Node::Parent { value, .. } => value,
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::Empty => 0,
            Node::Tail { .. } => 1,
            Node::Parent { next, .. } => 1 + next.size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use speculoos::assert_that;

    use super::*;

    #[test]
    fn converts_vec_into_linked_list() {
        assert_that(&LinkedList::from_vec(vec![1, 2, 3]).to_vec()).is_equal_to(vec![1, 2, 3]);
    }

    #[test]
    fn pushes_to_empty_list() {
        let mut under_test = LinkedList::new();

        assert_that(&under_test.size()).is_equal_to(0);

        under_test.push(1);

        assert_that(&under_test).is_equal_to(LinkedList::from_vec(vec![1]));
        assert_that(&under_test.size()).is_equal_to(1);
    }

    #[test]
    fn pushes_to_singleton_list() {
        let mut under_test = LinkedList::from_vec(vec![1]);

        assert_that(&under_test.size()).is_equal_to(1);

        under_test.push(2);

        assert_that(&under_test).is_equal_to(LinkedList::from_vec(vec![1, 2]));
        assert_that(&under_test.size()).is_equal_to(2);
    }

    #[test]
    fn pushes_to_multi_list() {
        let mut under_test = LinkedList::from_vec(vec![1, 2]);

        assert_that(&under_test.size()).is_equal_to(2);

        under_test.push(3);

        assert_that(&under_test).is_equal_to(LinkedList::from_vec(vec![1, 2, 3]));
        assert_that(&under_test.size()).is_equal_to(3);
    }
}
