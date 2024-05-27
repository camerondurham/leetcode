class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        # brute force validate each row / col contains things only once
        for i in range(9):
            # for each col
            cols = set()
            rows = set()
            # for each item in column i in board
            for j in range(9):
                # check row, if not '.', check if in set 1-9 and not in set
                cur_col = board[j][i]
                if not isInRangeAndNotRepeated(cur_col, cols):
                    return False
                # otherwise valid so add to the set
                cols.add(cur_col)

                cur_row = board[i][j]
                if not isInRangeAndNotRepeated(cur_row, rows):
                    return False
                rows.add(cur_row)
        
        # now validate each boxy thing
        for i in range(3):
            for j in range(3):
                start_row = i*3
                start_col = j*3
                box = set()
                for ii in range(start_row, start_row+3):
                    for jj in range(start_col, start_col+3):
                        if not isInRangeAndNotRepeated(board[ii][jj], box):
                            return False
                        box.add(board[ii][jj])
        return True


def isInRangeAndNotRepeated(num: str, existing: Set[int]) -> bool:
    return num == '.' or (int(num) >= 1 and int(num) <= 9 and num not in existing)

