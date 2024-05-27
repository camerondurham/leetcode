import heapq
class BasicSolution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        m = defaultdict(int) # keep track of the N most frequent items in the list
        heap = []

        # update the heap with the number of items in the list
        # heap stores a tuple: (priority=num_freq, value=num)
        for n in nums:
            prev=m.get(n, 0)
            m[n] = prev+1 # update count
        l = [(v,k) for (k,v) in m.items()] # the list so that the value / count becomes the key
        heapq.heapify(l) # transform into a heap, O(n)
        top = heapq.nlargest(k, l)
        answ = [] 
        for (k,v) in top:
            answ.append(v) # add the answer to the array
        return answ

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = {}
        # used for "bucket sort"
        # length can be at most len(nums) since that's the most frequent it can occur in the array
        freq = [[] for i in range(len(nums) + 1)]

        for n in nums:
            count[n] = 1 + count.get(n, 0)
        # build the array where the index = frequency occurred, value(s) are numbers that occur that frequently
        for n, c in count.items():
            freq[c].append(n)
        res = []
        for i in range(len(freq) - 1, 0, -1):
            for n in freq[i]:
                res.append(n)
                if len(res) == k:
                    return res

        # complexity of this is O(n) to get frequency count, O(n) to go through and find the top K most frequent
        # so total complexity is O(n)
        # space complexity is also O(n)
