class MyQueue {

    private Stack<Integer> stackA = new Stack<Integer>();
    private Stack<Integer> stackB = new Stack<Integer>();
    public MyQueue() {
    }
    
    public void push(int x) {
        stackA.push(x);
    }
    
    public int pop() {
        this.move();
        return stackB.pop();
    }
    
    public int peek() {
        this.move();
        return stackB.peek();
    }
    
    public boolean empty() {
        return stackA.empty() && stackB.empty();
    }
    
    private void move() {
        if (stackB.empty()) {
             while (!stackA.empty()) {
                int last = stackA.pop();
                stackB.push(last);
            }
        }
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue obj = new MyQueue();
 * obj.push(x);
 * int param_2 = obj.pop();
 * int param_3 = obj.peek();
 * boolean param_4 = obj.empty();
 */
