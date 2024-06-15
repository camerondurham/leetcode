class Solution {
    public List<String> generateParenthesis(int n) {
        // what is a valid answer ? equal num start end parens +  each pair is closed
        List<String> answer = new ArrayList();
        helper(0, 0, n, "", answer);
        return answer;
    }
    private void helper(int open, int close, int n, String s, List<String> res) {
        if (open + close > n * 2 || open > n) {
            return;
        }
        if (open == close && s.length() == n*2) {
            res.add(s);
            return;
        }
        if (open < n) {
            helper(open + 1, close, n, s + "(", res);
        }
        if (close < open) {
            helper(open, close + 1, n, s + ")", res);
        }
    }
}

/*

(  -> ()
) -> noop

() -> ()(), (()), ())), ((()

*/
