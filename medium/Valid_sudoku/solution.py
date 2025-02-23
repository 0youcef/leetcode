from typing import List


def isValidSudoku(self, board: List[List[str]]) -> bool:
    rows = [set() for _ in range(9)]
    colums = [set() for _ in range(9)]
    squares = [set() for _ in range(9)]
    k = 0
    for l in board:
        j = -1
        for i in l:
            j += 1
            if i == ".":
                continue

            if i in rows[k]:
                return False
            rows[k].add(i)

            if i in colums[j]:
                return False
            colums[j].add(i)

            if i in squares[(j // 3) + 3 * (k // 3)]:
                return False
            squares[(j // 3) + 3 * (k // 3)].add(i)
        k += 1
    return True


board = [
    ["5", "3", ".", ".", "7", ".", ".", ".", "."],
    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    [".", "9", "8", ".", ".", ".", ".", "6", "."],
    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    [".", "6", ".", ".", ".", ".", "2", "8", "."],
    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    [".", ".", ".", ".", "8", ".", ".", "7", "9"],
]
print(isValidSudoku(0, board))
