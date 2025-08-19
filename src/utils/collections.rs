pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Self>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn find<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        // Handle removing head node
        if let Some(ref head_node) = self.head {
            if predicate(&head_node.data) {
                return self.pop_front();
            }
        }

        // Handle removing from middle or end
        let mut current = &mut self.head;
        while let Some(node) = current {
            if let Some(ref next_node) = node.next {
                if predicate(&next_node.data) {
                    let removed_node = node.next.take().unwrap();
                    node.next = removed_node.next;
                    return Some(removed_node.data);
                }
            }
            current = &mut node.next;
        }
        None
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev
    }
}

impl<T> std::fmt::Display for LinkedList<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        let mut first = true;

        while let Some(node) = current {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.data)?;
            first = false;
            current = &node.next;
        }

        if first {
            write!(f, "empty")?;
        }

        Ok(())
    }
}
