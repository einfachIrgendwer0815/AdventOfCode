import sys
file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        lines.append(list(map(int, line.strip())))

#lines = [[2,1,9,9,9,4,3,2,1,0],[3,9,8,7,8,9,4,9,2,1],[9,8,5,6,7,8,9,8,9,2],[8,7,6,7,8,9,6,7,8,9],[9,8,9,9,9,6,5,6,7,8]]

def pointsToAdd(lines, point):
    toAdd = []
    if point[1] > 0:
        toAdd.append((point[0], point[1]-1))

    if point[1] < len(lines)-1:
        toAdd.append((point[0], point[1]+1))

    if point[0] > 0:
        toAdd.append((point[0]-1, point[1]))

    if point[0] < len(lines[point[1]])-1:
        toAdd.append((point[0]+1, point[1]))

    return toAdd

def calcSize(lines, start):
    size = 1

    searchedPoints = [start]
    searchQueue = []

    searchQueue += pointsToAdd(lines, start)

    while len(searchQueue) > 0:
        if searchQueue[0] not in searchedPoints and lines[searchQueue[0][1]][searchQueue[0][0]] < 9:
            size += 1
            searchedPoints.append(searchQueue[0])

            searchQueue += pointsToAdd(lines, searchQueue[0])

        searchQueue.pop(0)

    return size


greatest = []

for yi, y in enumerate(lines):
    for xi, x in enumerate(y):
        if xi > 0:
            if y[xi-1] <= x:
                continue

        if xi < len(y)-1:
            if y[xi+1] <= x:
                continue

        if yi > 0:
            if lines[yi-1][xi] <= x:
                continue

        if yi < len(lines)-1:
            if lines[yi+1][xi] <= x:
                continue

        size = calcSize(lines, (xi,yi))

        if len(greatest) < 3:
            greatest.append(size)
            greatest.sort(reverse=True)

        else:
            greatest.append(size)
            greatest.sort(reverse=True)
            greatest.pop()


print(greatest[0]*greatest[1]*greatest[2])
