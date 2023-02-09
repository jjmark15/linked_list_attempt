#[derive(Debug, Eq, PartialEq)]
pub struct LinkedList<T> {
    node: Node<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { node: Node::Empty }
    }

    pub fn from<I: IntoIterator<Item = T>>(it: I) -> Self {
        let mut list = LinkedList::new();

        it.into_iter().for_each(|v| list.push(v));

        list
    }

    pub fn to_vec(mut self) -> Vec<T> {
        let mut vec = vec![];

        while let Some(value) = self.node.pop_front() {
            vec.push(value);
        }

        vec
    }

    pub fn push(&mut self, val: T) {
        self.node.push(val);
    }

    pub fn push_front(&mut self, val: T) {
        self.node.push_front(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.node.pop()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.node.pop_front()
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

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node::Empty
    }
}

impl<T> Node<T> {
    fn push(&mut self, val: T) {
        match self {
            Node::Empty => *self = Node::Tail { value: val },
            Node::Tail { .. } => self.to_parent(val),
            Node::Parent { next, .. } => next.push(val),
        };
    }

    fn push_front(&mut self, val: T) {
        if self.is_empty() {
            return self.push(val);
        }

        let new_child = std::mem::take(self);

        *self = Node::Parent {
            value: val,
            next: Box::new(new_child),
        };
    }

    fn pop(&mut self) -> Option<T> {
        match self {
            Node::Empty => None,
            Node::Tail { .. } => Some(self.to_empty()),
            Node::Parent { next, .. } => {
                if next.is_tail() {
                    Some(self.to_tail())
                } else {
                    next.pop()
                }
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        match self {
            Node::Empty => None,
            Node::Tail { .. } => Some(self.to_empty()),
            Node::Parent { next, .. } => {
                let new_self = std::mem::take(next);

                let mut old_self = new_self;
                std::mem::swap(self, &mut old_self);

                Some(old_self.value())
            }
        }
    }

    fn to_parent(&mut self, child_value: T) {
        *self = Node::Parent {
            value: self.to_empty(),
            next: Box::new(Node::Tail { value: child_value }),
        };
    }

    fn to_tail(&mut self) -> T {
        let popped_val = self.next().unwrap().to_empty();

        *self = Node::Tail {
            value: self.to_empty(),
        };

        popped_val
    }

    fn to_empty(&mut self) -> T {
        std::mem::take(self).value()
    }

    fn is_tail(&self) -> bool {
        matches!(self, Node::Tail { .. })
    }

    fn is_empty(&self) -> bool {
        matches!(self, Node::Empty)
    }

    fn value(self) -> T {
        match self {
            Node::Empty => panic!("expected value node"),
            Node::Tail { value } => value,
            Node::Parent { value, .. } => value,
        }
    }

    fn next(&mut self) -> Option<&mut Self> {
        if let Node::Parent { next, .. } = self {
            return Some(next);
        }
        None
    }

    fn size(&self) -> usize {
        match self {
            Node::Empty => 0,
            Node::Tail { .. } => 1,
            Node::Parent { next, .. } => 1 + next.size(),
        }
    }
}

impl<V> FromIterator<V> for LinkedList<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        LinkedList::from(iter)
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn converts_iterator_into_linked_list() {
        assert_that(&LinkedList::from(vec![1, 2, 3].into_iter()).to_vec())
            .is_equal_to(vec![1, 2, 3]);
    }

    #[test]
    fn converts_iterator_trait_into_linked_list() {
        assert_that(&LinkedList::from_iter(vec![1, 2, 3].into_iter()).to_vec())
            .is_equal_to(vec![1, 2, 3]);
    }

    #[test]
    fn collects_from_iterator() {
        assert_that(&vec![1, 2].into_iter().collect()).is_equal_to(LinkedList::from(vec![1, 2]));
    }

    #[test]
    fn converts_vec_into_linked_list() {
        assert_that(&LinkedList::from(vec![1, 2, 3]).to_vec()).is_equal_to(vec![1, 2, 3]);
    }

    #[test]
    fn pushes_to_empty_list() {
        let mut under_test = LinkedList::new();

        assert_that(&under_test.size()).is_equal_to(0);

        under_test.push(1);

        assert_that(&under_test).is_equal_to(LinkedList::from(vec![1]));
        assert_that(&under_test.size()).is_equal_to(1);
    }

    #[test]
    fn pushes_to_singleton_list() {
        let mut under_test = LinkedList::from(vec![1]);

        assert_that(&under_test.size()).is_equal_to(1);

        under_test.push(2);

        assert_that(&under_test).is_equal_to(LinkedList::from(vec![1, 2]));
        assert_that(&under_test.size()).is_equal_to(2);
    }

    #[test]
    fn pushes_to_multi_list() {
        let mut under_test = LinkedList::from(vec![1, 2]);

        assert_that(&under_test.size()).is_equal_to(2);

        under_test.push(3);

        assert_that(&under_test).is_equal_to(LinkedList::from(vec![1, 2, 3]));
        assert_that(&under_test.size()).is_equal_to(3);
    }

    #[test]
    fn pushes_to_front_of_empty_list() {
        let mut under_test = LinkedList::new();

        assert_that(&under_test.size()).is_equal_to(0);

        under_test.push_front(1);

        assert_that(&under_test).is_equal_to(LinkedList::from(vec![1]));
        assert_that(&under_test.size()).is_equal_to(1);
    }

    #[test]
    fn pushes_to_front_of_singleton_list() {
        let mut under_test = LinkedList::from(vec![1]);

        assert_that(&under_test.size()).is_equal_to(1);

        under_test.push_front(2);

        assert_that(&under_test).is_equal_to(LinkedList::from(vec![2, 1]));
        assert_that(&under_test.size()).is_equal_to(2);
    }

    #[test]
    fn pushes_to_front_of_multi_list() {
        let mut under_test = LinkedList::from(vec![1, 2]);

        assert_that(&under_test.size()).is_equal_to(2);

        under_test.push_front(3);

        assert_that(&under_test).is_equal_to(LinkedList::from(vec![3, 1, 2]));
        assert_that(&under_test.size()).is_equal_to(3);
    }

    #[test]
    fn returns_empty_when_empty_list_is_popped() {
        let mut under_test: LinkedList<i32> = LinkedList::new();

        assert_that(&under_test.pop()).is_none();
    }

    #[test]
    fn returns_value_when_singleton_list_is_popped() {
        let mut under_test = LinkedList::from(vec![1]);

        assert_that(&under_test.size()).is_equal_to(1);
        assert_that(&under_test.pop()).contains(1);
        assert_that(&under_test.size()).is_equal_to(0);
    }

    #[test]
    fn returns_value_when_multi_list_is_popped() {
        let mut under_test = LinkedList::from(vec![1, 2]);

        assert_that(&under_test.size()).is_equal_to(2);
        assert_that(&under_test.pop()).contains(2);
        assert_that(&under_test.size()).is_equal_to(1);
    }

    #[test]
    fn returns_empty_when_empty_list_is_front_popped() {
        let mut under_test: LinkedList<i32> = LinkedList::new();

        assert_that(&under_test.pop_front()).is_none();
    }

    #[test]
    fn returns_value_when_singleton_list_is_front_popped() {
        let mut under_test = LinkedList::from(vec![1]);

        assert_that(&under_test.size()).is_equal_to(1);
        assert_that(&under_test.pop_front()).contains(1);
        assert_that(&under_test.size()).is_equal_to(0);
    }

    #[test]
    fn returns_value_when_multi_list_is_front_popped() {
        let mut under_test = LinkedList::from(vec![1, 2]);

        assert_that(&under_test.size()).is_equal_to(2);
        assert_that(&under_test.pop_front()).contains(1);
        assert_that(&under_test.size()).is_equal_to(1);
    }
}
