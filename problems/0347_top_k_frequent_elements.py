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
