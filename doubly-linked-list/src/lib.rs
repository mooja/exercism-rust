// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    prev: Option<Rc<RefCell<Node<T>>>>,
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T> {
    front: Option<Rc<RefCell<Node<T>>>>,
    back: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

pub struct Cursor<'a, T> {
    rc: Option<Rc<RefCell<Node<T>>>>,
    ll: &'a mut LinkedList<T>,
}

use std::marker::PhantomData;

pub struct Iter<'a, T> {
    node: Option<Rc<RefCell<Node<T>>>>,
    phantom: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            rc: self.front.clone(),
            ll: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            rc: self.back.clone(),
            ll: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            node: self.front.clone(),
            phantom: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.rc.as_ref().map(|rc| &mut (*rc.as_ptr()).data)
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.rc = self.rc.clone()?.borrow().next.clone();
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.rc = match &self.rc {
            Some(rc) => match rc.borrow().prev {
                Some(ref prev_cell_rc) => Some(prev_cell_rc.clone()),

                None => None,
            },

            None => None,
        };

        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.rc.is_none() {
            return None;
        }

        let (prev, next) = match &self.rc {
            None => (None, None),
            Some(rc) => (rc.borrow().prev.clone(), rc.borrow().next.clone()),
        };

        if let Some(prev_ptr) = &prev {
            (*prev_ptr.borrow_mut()).next = next.clone();
        } else {
            self.ll.front = next.clone();
        }

        if let Some(next_ptr) = &next {
            (*next_ptr.borrow_mut()).prev = prev.clone();
        } else {
            self.ll.back = prev.clone();
        }

        let rc = self.rc.take().unwrap();
        if (*rc.borrow()).next.is_some() {
            self.rc = (*rc.borrow()).next.clone();
        } else if (*rc.borrow()).prev.is_some() {
            self.rc = (*rc.borrow()).prev.clone();
        }

        let cell = match Rc::try_unwrap(rc) {
            Ok(cell) => cell,
            Err(rc) => panic!("Dangling pointer. Strong count: {}", Rc::strong_count(&rc)),
        };
        let node = cell.into_inner();

        self.ll.len -= 1;
        Some(node.data)
    }

    pub fn insert_after(&mut self, element: T) {
        // allocate new node
        let new_node = Rc::new(RefCell::new(Node {
            prev: None,
            data: element,
            next: None,
        }));

        match self.rc {
            None => {
                self.rc = Some(new_node.clone());
                self.ll.front = Some(new_node.clone());
                self.ll.back = Some(new_node.clone());
            }

            Some(ref cursor) => {
                let next = cursor.borrow().next.clone();
                if let Some(ref next) = next {
                    (*next.borrow_mut()).prev = Some(new_node.clone());
                    (*new_node.borrow_mut()).next = Some(next.clone());
                } else {
                    self.ll.back = Some(new_node.clone());
                }

                (*cursor.borrow_mut()).next = Some(new_node.clone());
                (*new_node.borrow_mut()).prev = Some(cursor.clone());
            }
        };

        self.ll.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        // allocate new node
        let new_node = Rc::new(RefCell::new(Node {
            prev: None,
            data: element,
            next: None,
        }));

        match self.rc {
            None => {
                self.rc = Some(new_node.clone());
                self.ll.front = Some(new_node.clone());
                self.ll.back = Some(new_node.clone());
            }

            Some(ref cursor) => {
                let prev = cursor.borrow().prev.clone();
                if let Some(ref prev_rc) = prev {
                    (*prev_rc.borrow_mut()).next = Some(new_node.clone());
                    (*new_node.borrow_mut()).prev = Some(prev_rc.clone());
                } else {
                    self.ll.front = Some(new_node.clone());
                }

                (*cursor.borrow_mut()).prev = Some(new_node.clone());
                (*new_node.borrow_mut()).next = Some(cursor.clone());
            }
        };

        self.ll.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let old_node = self.node.take()?;
        self.node = old_node.borrow().next.clone();
        unsafe { Some(&(*old_node.as_ptr()).data) }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while let Some(_) = cursor.take() {}
    }
}
