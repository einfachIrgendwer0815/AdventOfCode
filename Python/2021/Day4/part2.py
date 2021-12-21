file = "input.txt"

boards = []
with open(file, "r") as open_file:
    randomNumbers = tuple(map(int, open_file.readline().strip().split(',')))

    board = []
    while (line := open_file.readline()):
        formattedLine = list(map(int, line.strip().split()))

        if len(formattedLine) == 0:
            if len(board) >= 5:
                boards.append(board.copy())
                board.clear()

        else:
            board.append(formattedLine)


stop = False
finamNum = 0
for num in randomNumbers:
    copyBoards = boards.copy()
    for i, board in enumerate(boards):
        markedInRows = [ 0 for _ in range(5) ]
        markedInColumns = [ 0 for _ in range(5) ]

        for j, row in enumerate(board):
            for k, column in enumerate(row):
                if column == num:
                    boards[i][j][k] = -1
                if boards[i][j][k] == -1:
                    markedInRows[j] += 1
                    markedInColumns[k] += 1

        if 5 in markedInRows or 5 in markedInColumns:
            if len(copyBoards) > 1:
                copyBoards.remove(board)
            else:
                finalNum = num
                stop = True

    boards = copyBoards
    if stop:
        break

print(boards)
sum = 0

for row in boards[0]:
    for column in row:
        if column != -1:
            sum += column

print(sum * finalNum)
