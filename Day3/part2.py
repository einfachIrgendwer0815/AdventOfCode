file = "input.txt"

values = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        values.append(
            line.strip()
        )

valuesCopy = values.copy()

oxygen = 0

for i in range(len(values[0])):
    ones = 0
    zeros = 0

    for j in range(len(values)):
        if values[j][i] == "1":
            ones += 1
        else:
            zeros += 1

    if ones >= zeros:
        keepWith = "1"
    else:
        keepWith = "0"

    for value in values.copy():
        if value[i] != keepWith:
            values.remove(value)

    if len(values) == 1:
        break

oxygen = int(values[0],2)

values = valuesCopy.copy()

co2 = 0
for i in range(len(values[0])):
    ones = 0
    zeros = 0

    for j in range(len(values)):
        if values[j][i] == "1":
            ones += 1
        else:
            zeros += 1

    if ones < zeros:
        keepWith = "1"
    else:
        keepWith = "0"

    for value in values.copy():
        if value[i] != keepWith:
            values.remove(value)

    if len(values) == 1:
        break

co2 = int(values[0], 2)
print(oxygen*co2)
