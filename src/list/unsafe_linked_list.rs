use std::alloc::{handle_alloc_error, Allocator, Global, Layout};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

pub struct LinkedList<T: Clone> {
    head: *mut Node<T>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<'a, T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null::<Node<T>>() as *mut _,
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

        // SAFETY: allocation has been successful, so ptr is guaranteed to be
        // non-null, thus we can write to it
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

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.len == 0 {
                return None;
            } else if self.len == 1 {
                let result = Some((*self.head).value.clone());
                self.len = 0;
                let c: NonNull<Node<T>> = (&*self.head).into();
                Global.deallocate(c.cast(), Layout::new::<Node<T>>());

                return result;
            }

            let mut prev = self.head;
            // SAFETY: self.head can't be null or we would have returned None
            let mut current = (*self.head).next;

            while (*current).next != ptr::null_mut() {
                prev = current;

                // SAFETY: current is not null or we would have exited the loop
                current = (*current).next;
            }

            (*prev).next = std::ptr::null_mut();
            let result = Some((*current).value.clone());
            self.len -= 1;

            let c: NonNull<Node<T>> = (&*current).into();
            Global.deallocate(c.cast(), Layout::new::<Node<T>>());
            result
        }
    }

    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.len > 0 {
            self.pop();
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
            let value = &(*self.current).value as *const T;
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
    pub fn linked_list_pop() {
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        linked_list.push(5);
        linked_list.push(12);
        linked_list.push(23);

        assert_eq!(linked_list.pop(), Some(23));
        assert_eq!(linked_list.len(), 2);
        assert_eq!(linked_list.pop(), Some(12));
        assert_eq!(linked_list.len(), 1);
        assert_eq!(linked_list.pop(), Some(5));
        assert_eq!(linked_list.len(), 0);
        assert_eq!(linked_list.pop(), None);
        assert_eq!(linked_list.len(), 0);
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
