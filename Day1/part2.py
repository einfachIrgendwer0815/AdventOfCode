class Group():
    def __init__(self, length=3, index_offset=1):
        self.length = length
        self.index_offset = index_offset

        self.values = []

    def addValue(self, value):
        if self.isFull() == False:
            self.values.append(value)

    def isFull(self):
        return True if len(self.values) >= self.length else False

    def indexIsInGroup(self, index):
        offsetIndex = index + self.index_offset
        if (offsetIndex % 4) != 0:
            return True
        else:
            return False

file = "input.txt"

values = []
with open(file, "r") as open_file:
    while (line := open_file.readline()):
        values.append(
            int(line.strip())
        )

increases = 0

sums = []
activeGroups = []

for i in range(len(values)):
    if len(activeGroups) < 3:
        activeGroups.append([])

    for group in activeGroups:
        group.append(values[i])

    for group in activeGroups:
        if len(group) == 3:
            sums.append(group[0]+group[1]+group[2])
            activeGroups.remove(group)


for i in range(1, len(sums)):
    if sums[i] > sums[i-1]:
        increases += 1

print(increases)
