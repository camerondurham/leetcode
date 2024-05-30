class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        # search to find each row in the matrix
        ROWS, COLS = len(matrix), len(matrix[0])
        rstart, rend = 0, ROWS-1
        mid = 0
        while rstart <= rend:
            mid = (rstart+rend) // 2
            if matrix[mid][-1] < target:
                rstart =  mid + 1
            if matrix[mid][0] > target:
                rend = mid - 1
            if target >= matrix[mid][0] and target <= matrix[mid][-1]:
                break
        # check if we exited the loop because we went out of bounds
        if not rstart <= rend:
            return False
        cstart, cend = 0, COLS-1
        while cstart <= cend:
            cmid = (cstart + cend) // 2
            if matrix[mid][cmid] == target:
                return True
            elif matrix[mid][cmid] < target:
                cstart = cmid + 1
            elif matrix[mid][cmid] > target:
                cend = cmid - 1
        return False
