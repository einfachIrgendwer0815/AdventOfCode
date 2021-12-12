import itertools

file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        split = [list(i) for _, i in itertools.groupby(line.strip().split(), key='|'.__ne__)]
        split.pop(1)
        lines.append(split)

counter = 0
for line in lines:
    for i in line[1]:
        if len(i) in [2,4,3,7]:
            counter += 1
print(counter)
