from collections import defaultdict


class Solution:

    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        anagrams = defaultdict(list)
        for s in strs:
            sorted_s = ''.join(sorted(s))
            anagrams[sorted_s].append(s)
        return list(anagrams.values())


class OriginalSolution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        # group anagrams together based on which strings contain the same letters
        # figure out if two strings have the same set of characters, then group them together in a list

        # option with large time complexity ( N * (s log s), s=length of strings, N=length of list)
        # we sort each string, s log s
        # and we iterate through each item in the list
        # algorithm:
        # 1. sort words in the list
        # 2. if sorted order is the same, then group them together in a list
        answers = dict()
        for s in strs:
            sorted_s = ''.join(sorted(s))
            # add the string to these anagrams
            cur = answers.get(sorted_s, list())
            cur.append(s)
            answers[sorted_s] =  cur
        final_list = []
        for v_list in answers.values():
            final_list.append(v_list)
        return final_list

# optimal solution
class OptimalSolution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        res = defaultdict(list) # mapping charCount to a list of Anagrams
        for s in strs:
            count = [0] * 26 # lowercase english letters
            for c in s:
                count[ord(c) - ord('a')] += 1
            res[tuple(count)].append(s)
        return res.values()
        # Complexity O(m * n)
