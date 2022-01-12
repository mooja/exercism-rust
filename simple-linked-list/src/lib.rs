use std::iter::FromIterator;
use std::cell::{RefCell};
use std::rc::Rc;


pub struct Node <T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>
}

pub struct SimpleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: _element,
            next: None
        }));

        match self.size {
            0 => {
                self.head = Some(new_node);
            }

            _ => unsafe {
                let mut ll_node = self.head.as_ref().unwrap().as_ptr();
                let mut next = &(*ll_node).next;
                while next.is_some() {
                    ll_node = next.as_ref().unwrap().as_ptr();
                    next = &(*ll_node).next;
                }
                (*ll_node).next = Some(new_node);
            }
        }

        self.size += 1;
    }


    pub fn pop(&mut self) -> Option<T> {
        match self.size {
            0 => None,

            1 => {
                let refcell = match Rc::try_unwrap(self.head.take().unwrap()) {
                    Err(_) => panic!("too many references to head node"),
                    Ok(rc) => rc
                };
                self.size -= 1;
                Some(refcell.into_inner().data)
            }

            _ => unsafe {
                let mut ll_node = self.head.as_ref().unwrap().as_ptr();
                let mut next = &(*ll_node).next;

                for _ in 0..self.size - 2  {
                    ll_node = next.as_ref().unwrap().as_ptr();
                    next = &(*ll_node).next;
                }

                // ll_next now points to next to last node

                let last_node = match Rc::try_unwrap((*ll_node).next.take().unwrap()) {
                    Err(_) => panic!("too many refernces to last node while popping."),
                    Ok(rc) => rc.into_inner()
                };
                self.size -= 1;
                Some(last_node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,

            1 => unsafe {
                Some(&(*self.head.as_ref().unwrap().as_ptr()).data)
            }

            _ => unsafe {
                let mut node = self.head.as_ref().unwrap().clone();
                for _ in 0..self.size-1 {
                    let next = (*node.as_ptr()).next.as_ref().unwrap().clone();
                    node = next;
                }
                Some(&(*node.as_ptr()).data)
            }
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut self_ = self;
        let mut rv = SimpleLinkedList::new();
        while let Some(item) = self_.pop() {
            rv.push(item);
        }
        rv
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll: SimpleLinkedList<T> = SimpleLinkedList::new();

        for item in _iter {
            ll.push(item);
        }

        ll
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut self_ = self;
        let mut vec = vec![];

        while let Some(item) = self_.pop() {
            vec.push(item);
        }

        vec.reverse();
        vec
    }
}