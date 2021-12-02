//! This module defines a history data structure based with a possibly bounded capacity.

use std::collections::VecDeque;

/// A history is a queue with a possibly bounded capacity. A capacity of `None` indicates that the
/// length of the queue is unbounded, while a capacity of `Some(n)` indicates that the queue
/// contains at most `n` elements. If a history is at its capacity, the oldest element will be
/// dropped when a new element is added.
///
/// This interface is a bit barebones. However, the `History` type uses a [VecDeque] under the
/// hood and a method [History::as_deque] is provided to access this directly. Methods are also
/// provided to get the underlying `VecDeque` iterators. The elements in the `VecDeque` are ordered
/// from oldest to newest.
#[derive(Clone)]
pub struct History<T> {
    capacity: Option<usize>,
    deque: VecDeque<T>,
}

impl<T> History<T> {
    /// Create a new history with the given capacity.
    pub fn new(capacity: Option<usize>) -> Self {
        History {
            capacity,
            deque: History::deque_with_capacity(capacity),
        }
    }

    /// Get a reference to the underlying `VecDeque`. Elements are ordered from oldest to newest.
    pub fn as_deque(&self) -> &VecDeque<T> {
        &self.deque
    }

    /// Get the capacity of the history. A capacity of `None` indicates that the length of the
    /// queue is unbounded, while a capacity of `Some(n)` indicates that the queue contains at
    /// most `n` elements. If a history is at its capacity, the oldest element will be dropped
    /// when a new element is added.
    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }

    /// Set the capacity of the history, resizing the current history by forgetting elements if the
    /// new capacity is smaller than the old capacity.
    pub fn set_capacity(&mut self, new_capacity: Option<usize>) {
        if let Some(cap) = new_capacity {
            if self.deque.len() < cap {
                self.deque.reserve_exact(cap - self.deque.len());
            }
            while self.deque.len() > cap {
                self.deque.pop_front();
            }
        }
        self.capacity = new_capacity;
    }

    /// Add an element to the history, possibly dropping the oldest element if the history is at
    /// its capacity.
    pub fn add(&mut self, elem: T) {
        match self.capacity {
            Some(cap) => {
                if cap > 0 {
                    if self.deque.len() >= cap {
                        self.deque.pop_front();
                    }
                    self.deque.push_back(elem);
                }
            }
            None => self.deque.push_back(elem),
        };
    }

    /// Get the most recently added element from the history.
    pub fn newest(&self) -> Option<&T> {
        self.deque.back()
    }

    /// Get the number of elements stored in the history. This will be less than or equal to the
    /// capacity.
    pub fn len(&self) -> usize {
        self.deque.len()
    }

    /// Is the history empty?
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    /// Clear the history.
    pub fn clear(&mut self) {
        self.deque = History::deque_with_capacity(self.capacity);
    }

    /// Iterator over references to the elements, ordered oldest to newest.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.deque.iter()
    }

    /// Iterator over mutable references to the elements, ordered oldest to newest.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.deque.iter_mut()
    }

    /// Convert the history into an iterator over the elements, ordered oldest to newest.
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.deque.into_iter()
    }

    /// Create an empty `VecDeque` with the given capacity.
    fn deque_with_capacity(capacity: Option<usize>) -> VecDeque<T> {
        match capacity {
            Some(cap) => VecDeque::with_capacity(cap),
            None => VecDeque::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::History;

    #[test]
    fn finite_history() {
        let mut history = History::new(Some(5));
        assert!(history.is_empty());
        assert_eq!(0, history.len());
        assert_eq!(Some(5), history.capacity());

        history.add(3);
        history.add(4);
        history.add(5);
        assert_eq!(3, history.len());
        assert_eq!(Some(&5), history.newest());

        history.add(13);
        history.add(14);
        history.add(15);
        assert_eq!(5, history.len());
        assert_eq!(Some(&15), history.newest());
        assert_eq!(
            vec![4, 5, 13, 14, 15],
            history.into_iter().collect::<Vec<i32>>()
        );
    }

    #[test]
    fn resize_larger() {
        let mut history = History::new(Some(5));
        assert!(history.is_empty());
        assert_eq!(0, history.len());
        assert_eq!(Some(5), history.capacity());

        history.add(1);
        history.add(2);
        history.add(3);
        history.add(4);
        history.add(5);
        history.add(6);
        assert_eq!(5, history.len());
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.set_capacity(Some(7));
        assert_eq!(5, history.len());
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.add(7);
        history.add(8);
        history.add(9);
        assert_eq!(7, history.len());
        assert_eq!(
            vec![3, 4, 5, 6, 7, 8, 9],
            history.into_iter().collect::<Vec<i32>>()
        );
    }

    #[test]
    fn resize_smaller() {
        let mut history = History::new(Some(5));
        assert!(history.is_empty());
        assert_eq!(0, history.len());
        assert_eq!(Some(5), history.capacity());

        history.add(1);
        history.add(2);
        history.add(3);
        history.add(4);
        history.add(5);
        history.add(6);
        assert_eq!(5, history.len());
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.set_capacity(Some(3));
        assert_eq!(3, history.len());
        assert_eq!(
            vec![4, 5, 6],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.add(7);
        history.add(8);
        assert_eq!(3, history.len());
        assert_eq!(vec![6, 7, 8], history.into_iter().collect::<Vec<i32>>());
    }

    #[test]
    fn infinite_history() {
        let mut history = History::new(None);
        assert!(history.is_empty());
        assert_eq!(0, history.len());
        assert_eq!(None, history.capacity());

        history.add(3);
        history.add(4);
        history.add(5);
        assert_eq!(3, history.len());
        assert_eq!(Some(&5), history.newest());

        history.add(13);
        history.add(14);
        history.add(15);
        assert_eq!(6, history.len());
        assert_eq!(
            vec![3, 4, 5, 13, 14, 15],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.add(23);
        history.add(24);
        history.add(25);
        assert_eq!(9, history.len());
        assert_eq!(
            vec![3, 4, 5, 13, 14, 15, 23, 24, 25],
            history.into_iter().collect::<Vec<i32>>()
        );
    }

    #[test]
    fn make_finite() {
        let mut history = History::new(None);
        assert!(history.is_empty());
        assert_eq!(0, history.len());
        assert_eq!(None, history.capacity());

        history.add(3);
        history.add(4);
        history.add(5);
        assert_eq!(3, history.len());
        assert_eq!(Some(&5), history.newest());

        history.add(13);
        history.add(14);
        history.add(15);
        assert_eq!(6, history.len());
        assert_eq!(
            vec![3, 4, 5, 13, 14, 15],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.set_capacity(Some(4));
        assert_eq!(4, history.len());
        assert_eq!(
            vec![5, 13, 14, 15],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.add(23);
        history.add(24);
        history.add(25);
        assert_eq!(4, history.len());
        assert_eq!(
            vec![15, 23, 24, 25],
            history.into_iter().collect::<Vec<i32>>()
        );
    }

    #[test]
    fn make_infinite() {
        let mut history = History::new(Some(5));
        assert!(history.is_empty());
        assert_eq!(0, history.len());
        assert_eq!(Some(5), history.capacity());

        history.add(1);
        history.add(2);
        history.add(3);
        history.add(4);
        history.add(5);
        history.add(6);
        assert_eq!(5, history.len());
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.set_capacity(None);
        assert_eq!(5, history.len());
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            history.clone().into_iter().collect::<Vec<i32>>()
        );

        history.add(7);
        history.add(8);
        history.add(9);
        assert_eq!(8, history.len());
        assert_eq!(
            vec![2, 3, 4, 5, 6, 7, 8, 9],
            history.into_iter().collect::<Vec<i32>>()
        );
    }
}
