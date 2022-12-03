file = "input.txt"

currentElf = 0
topThree = [0, 0, 0]

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        if line.strip() == '':
            topThree.append(currentElf)
            topThree.sort(reverse=True)
            topThree.pop()
            currentElf = 0

            continue

        currentElf += int(line.strip())

print("Together the top three Elves have", sum(topThree), "calories.")
