file = "input.txt"

values = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        values.append(
            int(line.strip())
        )

increases = 0

sums = []
activeGroups = []

for i in range(len(values)):
    if len(activeGroups) < 3:
        activeGroups.append([])

    for group in activeGroups:
        group.append(values[i])

    for group in activeGroups:
        if len(group) == 3:
            sums.append(group[0]+group[1]+group[2])
            activeGroups.remove(group)


for i in range(1, len(sums)):
    if sums[i] > sums[i-1]:
        increases += 1

print(increases)
