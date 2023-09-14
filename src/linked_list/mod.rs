pub struct LinkedList<'a, T> {
    head: Option<&'a Node<'a, T>>,
    len: usize,
}

struct Node<'a, T> {
    value: T,
    next_value: Option<Box<Node<'a, T>>>,
}

impl<'a, T> Node<'a, T> {
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
        match self.head {
            Some(&head) => self.insert(value),
            None => self.create_head(value),
        }
        self.len += 1;
    }

    fn insert(&mut self, value: &'a T) {
        let new_element = Node::new(value);
        let new_element = Box::new(&new_element);

        self.head.unwrap().next_value = Some(new_element);
        self.head = new_element;
    }

    fn create_head(&mut self, value: &'a T) {
        let head_node = Node::new(value);
        self.head = Some(&head_node);
    }

    pub fn last(&self) -> &T {
        if self.len == 0 {
            panic!("Could not return last element. The list is currently empty.");
        }
        &(self.head.as_ref().unwrap().value)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
