use std::alloc::{handle_alloc_error, Allocator, Global, Layout};
use std::marker::PhantomData;
use std::ptr;

pub struct LinkedList<T> {
    head: *const Node<T>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<'a, T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null() as *const Node<T>,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { &(*self.head).value })
        }
    }

    pub fn push(&mut self, value: T) {
        let layout = Layout::new::<Node<T>>();
        let ptr = Global.allocate(layout);

        if ptr.is_err() {
            handle_alloc_error(layout);
        }

        let ptr = ptr.unwrap();
        let new_node = Node {
            value,
            next: std::ptr::null::<*mut Node<T>>() as *mut Node<T>,
        };

        // SAFETY: ptr is guaranteed to be not null, so we can write to it
        unsafe {
            ptr::write::<Node<T>>(ptr.as_ptr() as *mut Node<T>, new_node);

            // There is no head, let's create it
            if self.len == 0 {
                self.head = ptr.as_ptr() as *mut Node<T>;
            } else {
                let mut dest_node = self.head as *mut Node<T>;
                while (*dest_node).next != std::ptr::null_mut::<Node<T>>() {
                    dest_node = (*dest_node).next;
                }

                (*dest_node).next = ptr.as_ptr() as *mut Node<T>;
            }
        }

        self.len += 1;
    }

    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }
}

pub struct Iter<'a, T> {
    current: *const Node<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.current == std::ptr::null() {
            return None;
        }

        // SAFETY: self.current is not null since we just checked it
        unsafe {
            let value = &((*self.current).value) as *const T;
            self.current = (*self.current).next;
            Some(&*value)
        }
    }
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn linked_list_new() {
        let linked_list: LinkedList<i32> = LinkedList::new();
        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    pub fn linked_list_push() {
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.push(5);
        linked_list.push(12);
        linked_list.push(23);

        assert_eq!(linked_list.len(), 3);
        assert_eq!(linked_list.front(), Some(&5));
    }

    #[test]
    pub fn linked_list_iter() {
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.push(5);
        linked_list.push(12);
        linked_list.push(23);

        let mut iter = linked_list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&12));
        assert_eq!(iter.next(), Some(&23));
    }
}
