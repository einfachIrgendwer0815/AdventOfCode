file = "input.txt"

fish = []
with open(file, "r") as open_file:
    line = open_file.readline()
    fish = [ i for i in map(int, line.strip().split(',')) ]

newFish = fish.copy()
for _ in range(80):
    for i in range(len(fish)):
        if fish[i] > 0:
            newFish[i] = fish[i] - 1

        else:
            newFish[i] = 6
            newFish.append(8)
    fish = newFish.copy()

print(len(fish))
