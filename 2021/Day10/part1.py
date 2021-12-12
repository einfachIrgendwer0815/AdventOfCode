file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        lines.append(list(line.strip()))

invalidChars = {
    ')': 0,
    '}': 0,
    '>': 0,
    ']': 0
}

for line in lines:
    expectedChars = []

    for char in line:
        if char == '(':
            expectedChars.append(')')
        elif char == '{':
            expectedChars.append('}')
        elif char == '<':
            expectedChars.append('>')
        elif char == '[':
            expectedChars.append(']')

        if len(expectedChars) == 0:
            if char in invalidChars.keys():
                invalidChars[char] += 1

                break

        elif char in invalidChars.keys():
            if expectedChars[-1] != char:
                invalidChars[char] += 1

                break
            else:
                expectedChars.pop()

points = 0
points += invalidChars[')']*3
points += invalidChars[']']*57
points += invalidChars['}']*1197
points += invalidChars['>']*25137

print(points)
