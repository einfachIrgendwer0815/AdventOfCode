import statistics

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

linesCopy = lines.copy()

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

                if line in linesCopy:
                    linesCopy.remove(line)
                break

        elif char in invalidChars.keys():
            if expectedChars[-1] != char:
                invalidChars[char] += 1

                if line in linesCopy:
                    linesCopy.remove(line)
                break
            else:
                expectedChars.pop()

allScores = []
for line in linesCopy:
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
        elif char in ']}>)':
            expectedChars.pop()


    expectedChars = expectedChars[::-1]
    score = 0

    for char in expectedChars:
        score *= 5
        if char == ')':
            score += 1
        elif char == ']':
            score += 2
        elif char == '}':
            score += 3
        elif char == '>':
            score += 4

    allScores.append(score)

print(statistics.median(allScores))
