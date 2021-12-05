file = "input.txt"

lines = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        splitted = line.strip().split()
        splitted.pop(1)

        splitted[0] = tuple(map(int, splitted[0].split(',')))
        splitted[1] = tuple(map(int, splitted[1].split(',')))

        lines.append(tuple(splitted))


pos = {}
for line in lines:
    if line[0][0] == line[1][0]:
        coordRange = range(line[0][1], line[1][1]+1) if line[0][1] <= line[1][1] else range(line[1][1],line[0][1]+1)
        for i in coordRange:
            posCoord = (line[0][0], i)

            if posCoord not in pos.keys():
                pos[posCoord] = 1
            else:
                pos[posCoord] += 1
    elif line[0][1] == line[1][1]:
        coordRange = range(line[0][0], line[1][0]+1) if line[0][0] <= line[1][0] else range(line[1][0],line[0][0]+1)
        for i in coordRange:
            posCoord = (i, line[0][1])

            if posCoord not in pos.keys():
                pos[posCoord] = 1
            else:
                pos[posCoord] += 1

#print(pos)

over1 = 0

for posCoord in pos.keys():
    if pos[posCoord] > 1:
        over1 += 1

print(over1)
