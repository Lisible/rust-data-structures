use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<Node<T>>,
}

struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    value: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn head(&self) -> Option<&T> {
        // SAFETY: This is safe because node is a NonNull
        unsafe { self.head.map(|node| &(*node.as_ptr()).value) }
    }

    pub fn tail(&self) -> Option<&T> {
        // SAFETY: This is safe because node is a NonNull
        unsafe { self.tail.map(|node| &(*node.as_ptr()).value) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node = Box::leak(node);

        if self.tail == None {
            let node = NonNull::new(node as *mut _).unwrap();
            self.head = Some(node);
            self.tail = Some(node);
        } else {
            unsafe {
                // SAFETY: This is safe because we know that self.head is NonNull
                let current_tail = self.tail.unwrap();
                let node: NonNull<Node<T>> = NonNull::new(node as *mut _).unwrap();
                (*node.as_ptr()).prev = Some(current_tail);
                (*current_tail.as_ptr()).next = Some(node);
                self.tail = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node = Box::leak(node);

        if self.head == None {
            let node = NonNull::new(node as *mut _).unwrap();
            self.head = Some(node);
            self.tail = Some(node);
        } else {
            unsafe {
                // SAFETY: This is safe because we know that self.head is NonNull
                let current_head = self.head.unwrap();
                let node: NonNull<Node<T>> = NonNull::new(node as *mut _).unwrap();
                (*node.as_ptr()).next = Some(current_head);
                (*current_head.as_ptr()).prev = Some(node);
                self.head = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let head = self.head.unwrap();

            self.head = head.as_ref().next;
            if self.head != None {
                (*self.head.unwrap().as_ptr()).prev = None
            }

            let head = Box::from_raw(head.as_ptr());
            self.len -= 1;
            Some(head.value)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let tail = self.tail.unwrap();

            self.tail = tail.as_ref().prev;
            if self.tail != None {
                (*self.tail.unwrap().as_ptr()).next = None
            }

            let tail = Box::from_raw(tail.as_ptr());
            self.len -= 1;
            Some(tail.value)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.len > 0 {
            self.pop_back();
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            prev: None,
            next: None,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn linked_list_new() {
        let list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.head, None);
        assert_eq!(list.tail, None);
        assert_eq!(list.len, 0);
    }

    #[test]
    pub fn linked_list_head_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.head(), None);
    }

    #[test]
    pub fn linked_list_head_single() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(55);

        assert_eq!(list.head(), Some(&55));
    }

    #[test]
    pub fn linked_list_head_multiple() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.head(), Some(&1));
    }

    #[test]
    pub fn linked_list_tail_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.tail(), None);
    }

    #[test]
    pub fn linked_list_tail_single() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(55);

        assert_eq!(list.tail(), Some(&55));
    }

    #[test]
    pub fn linked_list_tail_multiple() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.tail(), Some(&3));
    }

    #[test]
    pub fn linked_list_push_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(4);
        list.push_front(12);
        list.push_front(14);

        assert_eq!(list.len, 3);
        assert_eq!(list.head(), Some(&14));
        assert_eq!(list.tail(), Some(&4));
    }

    #[test]
    pub fn linked_list_pop_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(4);
        list.push_back(12);
        list.push_back(14);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(12));

        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(14));

        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    pub fn linked_list_push_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(4);
        list.push_back(12);
        list.push_back(14);

        assert_eq!(list.len, 3);
        assert_eq!(list.head(), Some(&4));
        assert_eq!(list.tail(), Some(&14));
    }

    #[test]
    pub fn linked_list_pop_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(4);
        list.push_back(12);
        list.push_back(14);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(14));

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(12));

        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
    }
}
