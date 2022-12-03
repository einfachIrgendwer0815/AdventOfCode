file = "input.txt"

values = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        values.append(
            line.strip()
        )

mostCommonBits = ""
leastCommonBits = ""

ones = [ 0 for _ in range(len(values[0])) ]
zeros = [ 0 for _ in range(len(values[0])) ]

for i in range(len(values[0])):
    for j in range(len(values)):
        if values[j][i] == "1":
            ones[i] += 1
        else:
            zeros[i] += 1

for i in range(len(values[0])):
    if ones[i] > zeros[i]:
        mostCommonBits += "1"
        leastCommonBits += "0"
    else:
        mostCommonBits += "0"
        leastCommonBits += "1"


print(int(mostCommonBits,2)*int(leastCommonBits, 2))
