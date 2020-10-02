struct RecentCounter {
    stk: Vec<i32>,
    min: i32,
    max: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            stk:  Vec::<i32>::with_capacity(10000),
            min: 0,
            max: 0,
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.stk.push(t);
        self.max += 1;
        while(self.stk[self.min as usize] < (t - 3000)){self.min += 1;}
        self.max - self.min
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
