use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdManager {
    next_id: u32,
    max_id: u32,
}

impl IdManager {
    pub fn new(max_id: u32) -> Self {
        Self { next_id: 1, max_id }
    }

    pub fn get_max_id(&self) -> u32 {
        self.max_id
    }
}

impl Iterator for IdManager {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_id <= self.max_id {
            let id = self.next_id;
            self.next_id += 1;
            Some(id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_id_manager_new() {
        let id_manager = IdManager::new(10);
        assert_eq!(id_manager.next_id, 1);
        assert_eq!(id_manager.max_id, 10);
    }

    #[test]
    fn test_id_manager_next() {
        let mut id_manager = IdManager::new(10);
        assert_eq!(id_manager.next(), Some(1));
        assert_eq!(id_manager.next(), Some(2));
        assert_eq!(id_manager.next(), Some(3));
        assert_eq!(id_manager.next(), Some(4));
        assert_eq!(id_manager.next(), Some(5));
        assert_eq!(id_manager.next(), Some(6));
        assert_eq!(id_manager.next(), Some(7));
        assert_eq!(id_manager.next(), Some(8));
        assert_eq!(id_manager.next(), Some(9));
        assert_eq!(id_manager.next(), Some(10));
        assert_eq!(id_manager.next(), None);
    }

    #[test]
    fn test_id_manager_max_id() {
        let id_manager = IdManager::new(10);
        assert_eq!(id_manager.get_max_id(), 10);
    }

    #[test]
    fn test_id_manager_clone() {
        let mut id_manager = IdManager::new(10);
        let mut id_manager_clone = id_manager.clone();
        assert_eq!(id_manager.next(), Some(1));
        assert_eq!(id_manager_clone.next(), Some(1));
        assert_eq!(id_manager.next(), Some(2));
        assert_eq!(id_manager_clone.next(), Some(2));
    }

    #[test]
    fn test_id_manager_iter() {
        let id_iter: Vec<u32> = IdManager::new(10).into_iter().collect();
        assert_eq!(id_iter.len(), 10);
    }
}
