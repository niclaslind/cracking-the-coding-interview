#![allow(dead_code)]
#![feature(linked_list_remove)]

use std::{borrow::BorrowMut, rc::Rc};

/// Write code to remove duplicates from an unsorted linked list
///
/// FOLLOW UP:
///
/// How would you solve this problem if a temporary buffer is not allowed?
mod remove_duplicates;

/// Implement an algorigthm to find the kth to last element of a singly linked list.
mod return_kth_to_last;

/// Implement an algorigthm to delete a node in the middle (i.e. any node but the first and last node,
/// not necessarily teh middle) of a singly linked list, given only access to that node.
///
/// EXAMPLE:
///
/// Input: the nod c from the linked list a->b->c->d->e->f
///
/// Result: nothing is returned, but the new linked list looks like
/// a->b->d->e->f
mod delete_middle_node;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: Link<T>) -> Self {
        Self {
            data,
            next,
            prev: None,
        }
    }

    fn tail(node: &Link<T>) -> Link<T> {
        if let Some(current) = node.clone().unwrap().next.as_ref().cloned() {
            return Node::tail(&Some(current));
        }
        Some(node.clone().unwrap().clone())
    }
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn prepend(&self, data: T) -> List<T> {
        let head = Some(Rc::new(Node::new(data, self.head.clone())));

        List { head }
    }

    fn tail(&self) -> List<T> {
        let head = self.head.as_ref().and_then(|node| node.next.clone());
        List { head }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}
