file = "input.txt"

fishRaw = []
with open(file, "r") as open_file:
    line = open_file.readline()
    fishRaw = [ i for i in map(int, line.strip().split(',')) ]

fish = [0 for _ in range(9)]

for i in fishRaw:
    fish[i] += 1

#print(fish)
for round in range(256):
    newFish = fish[0]
    oldFish = fish[0]

    fish = fish[1:]
    fish.append(newFish)
    fish[6] += oldFish

    #print(fish)


fishSum = sum(fish)
print(fishSum)
