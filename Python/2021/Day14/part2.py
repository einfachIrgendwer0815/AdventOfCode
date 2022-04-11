def process(rules, counter):
    newCounter = {}

    for pair in counter.keys():
        inserted = rules[pair] if pair in rules.keys() else None

        if inserted != None:
            leftPair = pair[0] + inserted
            rightPair = inserted + pair[1]

            if leftPair not in newCounter.keys():
                newCounter[leftPair] = 0

            newCounter[leftPair] += counter[pair]

            if rightPair not in newCounter.keys():
                newCounter[rightPair] = 0

            newCounter[rightPair] += counter[pair]
        else:
            if pair not in newCounter.keys():
                newCounter[pair] = 0

            newCounter[pair] += counter[pair]

    return newCounter

template = None
rules = {}

file = 'input.txt'
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        if template == None:
            template = line.strip().split('->')[0]

        else:
            if len(line.strip()) > 0:
                splitUp = line.strip().split(' -> ')
                rules[splitUp[0]] = splitUp[1]

counter = {}
start = template[0]

for i in range(len(template) - 1):
    if template[i:i+2] not in counter.keys():
        counter[template[i:i+2]] = 0
    counter[template[i:i+2]] += 1

for _ in range(40):
    counter = process(rules, counter)

letters = {
    start: 1
}

for pair in counter:
    if pair[1] not in letters.keys():
        letters[pair[1]] = 0

    letters[pair[1]] += counter[pair]

maxCount = max(letters.values())
minCount = min(letters.values())

print(maxCount - minCount)
