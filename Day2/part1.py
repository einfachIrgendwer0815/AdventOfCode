file = "input.txt"

inputs = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        inputs.append(
            line.strip().split()
        )

horizontal = 0
depth = 0

for input in inputs:
    if input[0] == "forward":
        horizontal += int(input[1])

    elif input[0] == "down":
        depth += int(input[1])

    elif input[0] == "up":
        depth -= int(input[1])

print(horizontal*depth)
