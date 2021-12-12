file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        lines.append(list(map(int, line.strip())))

riskSum = 0

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

        riskSum += x + 1

print(riskSum)
