class MinStack {
    private final List<Integer> stack = new ArrayList<>();
    private final List<Integer> minNumberStack = new ArrayList<>();
    private Integer minValue;

    public MinStack() {
        this.minValue = null;
    }

    public void push(int val) {
        stack.add(val);
        minValue = minValue == null ? val : Math.min(minValue, val);
        minNumberStack.add(minValue);
    }

    public void pop() {
        int index = stack.size() - 1;
        minNumberStack.remove(index);
        stack.remove(index);
        minValue = minNumberStack.isEmpty() ? null : minNumberStack.get(stack.size() - 1);
    }

    public int top() {
        // what if empty?
        if (stack.isEmpty()) {
            throw new RuntimeException("Invalid operation");
        }
        return stack.get(stack.size() - 1);
    }

    public int getMin() {
        if (stack.isEmpty() || minValue == null) {
            throw new RuntimeException("Invalid operation");
        } else {
            return minNumberStack.get(minNumberStack.size() - 1);
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack obj = new MinStack();
 * obj.push(val);
 * obj.pop();
 * int param_3 = obj.top();
 * int param_4 = obj.getMin();
 */
