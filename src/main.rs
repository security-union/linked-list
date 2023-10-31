// Define a basic Node structure
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Define a LinkedList structure
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Add an element to the front of the linked list
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove and return the element from the front of the linked list
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    // Check if the linked list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Get the length of the linked list
    pub fn len(&self) -> usize {
        let mut current = &self.head;
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}


fn main() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("Length of the linked list: {}", list.len());

    while let Some(value) = list.pop() {
        println!("Popped value: {}", value);
    }

    println!("Is the linked list empty? {}", list.is_empty());
}
