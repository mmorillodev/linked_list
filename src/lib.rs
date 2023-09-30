pub struct LinkedList<'a, T> {
    head: Option<Node<&'a T>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next_value: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next_value: None,
        }
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> Self {
        Self {
            head: Option::None,
            len: 0,
        }
    }

    pub fn push(&mut self, value: &'a T) {
        match &self.head {
            Some(_) => self.add_el_to_head(value),
            None => self.add_head(value),
        };

        self.len += 1;
    }

    fn add_el_to_head(&mut self, value: &'a T) {
        let new_value = Box::new(Node::new(value));

        self.head.as_mut().unwrap().next_value = Some(new_value);
        self.head = Some(Node::new(value));
    }

    fn add_head(&mut self, value: &'a T) {
        self.head = Some(Node::new(value));
    }

    pub fn last(&self) -> Option<&T> {
        match &self.head {
            Some(head) => Some(head.value),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new_empty_list() {
        let linked_list: LinkedList<i32> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);
        assert_eq!(linked_list.last(), None)
    }

    #[test]
    fn push_to_the_list() {
        let mut linked_list = LinkedList::<i32>::new();
        let expected_value = 3;

        linked_list.push(&expected_value);

        assert_eq!(linked_list.len(), 1);
        assert_eq!(linked_list.last().unwrap(), &expected_value);

        let expected_value = 5;

        linked_list.push(&expected_value);

        assert_eq!(linked_list.len(), 2);
        assert_eq!(linked_list.last().unwrap(), &expected_value);
    }
}
