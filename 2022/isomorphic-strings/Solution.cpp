/*
205. Isomorphic Strings
Easy

Given two strings s and t, determine if they are isomorphic.

Two strings s and t are isomorphic if the characters in s can be replaced to get t.

All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

 

Example 1:

Input: s = "egg", t = "add"
Output: true

Example 2:

Input: s = "foo", t = "bar"
Output: false

foo
baz


Example 3:

Input: s = "paper", t = "title"
Output: true

papers
titles

Constraints:

    1 <= s.length <= 5 * 104
    t.length == s.length
    s and t consist of any valid ascii character.

Runtime: 6 ms, faster than 83.99% of C++ online submissions for Isomorphic Strings.
Memory Usage: 7 MB, less than 46.63% of C++ online submissions for Isomorphic Strings.

*/

/*
 * "Hint": know the definition of isomorphic. map from s -> t and t -> s 
 */

#include<iostream>
#include<string>
#include<unordered_map>
#include<unordered_set>
using std::string;
class Solution {
public:
    bool isIsomorphic(string s, string t) {
      if (s.size() != t.size()) return false;
      std::unordered_map<char,char> s_t;
      std::unordered_map<char,char> t_s;

      for (int i=0; i < s.size(); i++) {
        char curr = s[i];
        char in_t = t[i];
        bool exists_s_t = exists(s_t,curr);
        bool exists_t_s = exists(t_s, in_t);
        if (!exists_s_t && !exists_t_s) {
          s_t[curr] = in_t;
          t_s[in_t] = curr;
        } else if (!exists_s_t && exists_t_s) {
          return false;
        } else if (exists_s_t && !exists_t_s) {
          return false;
        } else if (s_t[curr] != in_t || t_s[in_t] != curr) {
          return false;
        }
      }
      return true;
    }

    bool exists(std::unordered_map<char,char>& m, char c) {
      return (m.find(c) != m.end());
    }
};
void test(std::string s, std::string t, Solution& sol) {
  std::cout << "s=" << s << ",t=" << t << ",";
  std::cout << "result=" << sol.isIsomorphic(s,t) <<"\n\n";
}
int main() {
  Solution s;
  test("egg","add",s);
  test("foo","bar",s);
  test("paper","title",s);
  test("papers","titles",s);
  test("foz","bar",s);
  test("badc","baba",s);

}
