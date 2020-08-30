struct MyHashSet {
    set: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            set: vec![false; 1_000_001],
        }
    }

    fn add(&mut self, key: i32) {
        if let Some(elem) = self.set.get_mut(key as usize) {
            *elem = true;
        }
    }

    fn remove(&mut self, key: i32) {
        if let Some(elem) = self.set.get_mut(key as usize) {
            *elem = false;
        }
    }

    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        if let Some(elem) = self.set.get(key as usize) {
            return *elem;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1() {
        let mut obj = MyHashSet::new();
        obj.add(4);
        obj.remove(4);
        assert_eq!(obj.contains(4), false);
    }

    #[test]
    fn ex2() {
        let mut obj = MyHashSet::new();
        obj.remove(1000);
        obj.add(1000);
        assert_eq!(obj.contains(1000), true);
    }

    #[test]
    fn ex3() {
        let key = 1_000_000;
        let mut obj = MyHashSet::new();
        obj.add(key);
        assert_eq!(obj.contains(key), true);
    }

    #[test]
    fn ex4() {
        let arr = vec![193, 28, 381, 21, 435, 13, 4];
        let mut obj = MyHashSet::new();
        for &i in arr.iter() {
            obj.add(i);
        }
        for &i in arr.iter() {
            assert_eq!(obj.contains(i), true);
            obj.remove(i);
            assert_eq!(obj.contains(i), false);
        }
    }
}

/*
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

/*
Example:

    MyHashSet hashSet = new MyHashSet();
    hashSet.add(1);
    hashSet.add(2);
    hashSet.contains(1);    // returns true
    hashSet.contains(3);    // returns false (not found)
    hashSet.add(2);
    hashSet.contains(2);    // returns true
    hashSet.remove(2);
    hashSet.contains(2);    // returns false (already removed)

Note:

    All values will be in the range of [0, 1000000].
    The number of operations will be in the range of [1, 10000].
    Please do not use the built-in HashSet library.

*/
