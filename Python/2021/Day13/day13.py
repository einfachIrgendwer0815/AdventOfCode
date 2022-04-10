def foldOnY(points, y):
    top = set()
    bottom = set()

    for point in points:
        if point[1] < y:
            top.add(point)
        elif point[1] > y:
            bottom.add(point)

    for point in bottom:
        newPoint = (point[0], point[1] - 2 * (point[1] - y))

        top.add(newPoint)

    return top

def foldOnX(points, x):
    left = set()
    right = set()

    for point in points:
        if point[0] < x:
            left.add(point)
        elif point[0] > x:
            right.add(point)

    for point in right:
        newPoint = (point[0] - 2 * (point[0] - x), point[1])

        left.add(newPoint)

    return left

file = "input.txt"

points = set()

folds = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        if len(line.strip()) == 0:
            continue

        if not "fold along" in line:
            points.add(tuple(map(int, line.strip().split(','))))
        else:
            splitUp = line.strip().split()[2].split('=')
            folds.append((splitUp[0], int(splitUp[1])))

for fold in folds:
    if fold[0] == 'x':
        points = foldOnX(points, fold[1])
    elif fold[0] == 'y':
        points = foldOnY(points, fold[1])

    if fold == folds[0]:
        print("Part 1:", len(points))

print("Part 2:")

maxX = 0
maxY = 0

for point in points:
    if point[0] > maxX:
        maxX = point[0]

    if point[1] > maxY:
        maxY = point[1]

for y in range(maxY+1):
    for x in range(maxX+1):
        print("#" if (x, y) in points else '.', end="")

    print('\n', end='')
