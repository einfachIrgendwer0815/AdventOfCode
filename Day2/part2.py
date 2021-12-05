file = "input.txt"

inputs = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        inputs.append(
            line.strip().split()
        )

horizontal = 0
depth = 0
aim = 0

for input in inputs:
    input[1] = int(input[1])
    if input[0] == "down":
        aim += input[1]

    elif input[0] == "up":
        aim -= input[1]

    elif input[0] == "forward":
        horizontal += input[1]
        depth += aim*input[1]

print(horizontal*depth)
