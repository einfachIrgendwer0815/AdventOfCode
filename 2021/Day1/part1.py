file = "input.txt"

values = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        values.append(
            int(line.strip())
        )

increases = 0

for i in range(1,len(values)):
    if values[i] > values[i-1]:
        increases += 1

print(increases)
