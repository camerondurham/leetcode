class NonWorkingSolution {
    // n=3
    // Output: ["((()))","(()())","(())()","()(())","()()()"]
    // n=2, 4 chars, 2 pairs
    // Output: ["(())", "()()"]
    // ()()
    public List<String> generateParenthesis(int n) {
        // set to generate the final answer from to eliminate duplicates

        Stack<String> parens = new Stack<>();
        int parenCount = 1;
        // base case
        parens.push("()");
        while (parenCount < n) {
            List<String> temp = new ArrayList<>();
            while (!parens.isEmpty()) {
                String top = parens.pop();
                temp.add(String.format("(%s)", top));
                temp.add(String.format("%s()", top));
                temp.add(String.format("()%s", top));
            }
            parens.addAll(temp);
            parenCount++;
        }
        Set<String> set = new HashSet<>();
        set.addAll(parens);
        return List.copyOf(set);
    }
}
