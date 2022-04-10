def processTemplate(template, rules):
    processed = ""

    for i in range(len(template) - 1):
        toInsert = rules[template[i:i+2]] if template[i:i+2] in rules.keys() else ''

        if i == 0:
            processed += template[i]

        processed += toInsert + template[i+1]

    return processed

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

for i in range(10):
    template = processTemplate(template, rules)

counts = {}

for letter in template:
    if letter in counts.keys():
        continue
    else:
        counts[letter] = template.count(letter)

max_count = max(counts.values())
min_count = min(counts.values())

print(max_count-min_count)
