file = "input.txt"

currentElf = 0
maxElf = 0

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        if line.strip() == '':
            maxElf = max(maxElf, currentElf)
            currentElf = 0

            continue

        currentElf += int(line.strip())

print("The Elf with the most calories has", maxElf, "calories.")
