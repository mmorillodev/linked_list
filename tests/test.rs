use linked_list::linked_list::LinkedList;

#[test]
fn create_new_empty_list() {
    let linked_list: LinkedList<i32> = LinkedList::new();

    assert_eq!(linked_list.len(), 0);
}

#[test]
fn push_to_the_list() {
    let mut linked_list = LinkedList::new();
    let expected_value = 3;

    linked_list.push(expected_value);

    assert_eq!(linked_list.len(), 1);
    assert_eq!(linked_list.last(), &expected_value);

    let expected_value = 5;

    linked_list.push(expected_value);

    assert_eq!(linked_list.len(), 2);
    assert_eq!(linked_list.last(), &expected_value);
}
