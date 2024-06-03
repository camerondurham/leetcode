class Solution {
public:
    bool isValid(string s) {

        if (s.size() % 2 != 0) {
            return false;
        }
        stack<char> parens;
        for (char c : s) {
            if (c == '(' || c == '[' || c == '{') {
                parens.push(c);
                continue;
            } else if ((c == ')' || c == '}' || c == ']') && parens.empty()) {
                return false;
            }
            char top = parens.top();
            if ((c == ')' && top == '(') || (c == ']' && top == '[') ||
                (c == '}' && top == '{')) {
                parens.pop();
            } else {
                return false;
            }
        }
        return parens.empty();
    }
};
