def adjacentPositions(xi, yi, lines):
    up = False
    down = False
    left = False
    right = False
    upLeft = False
    upRight = False
    downLeft = False
    downRight = False

    if yi > 0:
        up = True

    if yi < len(lines)-1:
        down = True

    if xi > 0:
        left = True

    if xi < len(lines[yi]) - 1:
        right = True

    if up and left: upLeft = True
    if up and right: upRight = True
    if down and left: downLeft = True
    if down and right: downRight = True

    return up, down, left, right, upLeft, upRight, downLeft, downRight

file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        lines.append(list(map(int, line.strip())))

counter = 1
while True:
    for y in lines:
        for i, x in enumerate(y):
            y[i] += 1

    noFlashes = False
    while not noFlashes:
        noFlashes = True
        for yi in range(len(lines)):
            for xi in range(len(lines[yi])):
                if lines[yi][xi] > 9:
                    noFlashes = False
                    lines[yi][xi] = -1

                    adjacents = adjacentPositions(xi, yi, lines)
                    if adjacents[0] and lines[yi-1][xi] > -1:
                        lines[yi-1][xi] += 1
                    if adjacents[1] and lines[yi+1][xi] > -1:
                        lines[yi+1][xi] += 1
                    if adjacents[2] and lines[yi][xi-1] > -1:
                        lines[yi][xi-1] += 1
                    if adjacents[3] and lines[yi][xi+1] > -1:
                        lines[yi][xi+1] += 1
                    if adjacents[4] and lines[yi-1][xi-1] > -1:
                        lines[yi-1][xi-1] += 1
                    if adjacents[5] and lines[yi-1][xi+1] > -1:
                        lines[yi-1][xi+1] += 1
                    if adjacents[6] and lines[yi+1][xi-1] > -1:
                        lines[yi+1][xi-1] += 1
                    if adjacents[7] and lines[yi+1][xi+1] >  -1:
                        lines[yi+1][xi+1] += 1

    for y in lines:
        for i, x in enumerate(y):
            if x == -1:
                y[i] = 0

    if sum(list(map(sum, lines))) == 0:
        print(counter)
        break

    counter += 1
