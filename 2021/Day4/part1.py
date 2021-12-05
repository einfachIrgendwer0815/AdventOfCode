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


winnerboard = []
finalNum = 0
stop = False
for num in randomNumbers:
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
            winnerboard = board.copy()
            finalNum = num
            stop = True
            break

    if stop:
        break

print(winnerboard)

sum = 0

for row in winnerboard:
    for column in row:
        if column != -1:
            sum += column

print(sum * finalNum)
