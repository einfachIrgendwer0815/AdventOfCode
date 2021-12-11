import statistics

file = "input.txt"

crabsPos = []
with open(file, "r") as open_file:
    line = open_file.readline()
    crabsPos = [ i for i in map(int, line.strip().split(',')) ]

#crabsPos = [16,1,2,0,4,2,7,1,2,14]

quer = int(statistics.mean(crabsPos))

fuel = 0
for crab in crabsPos:
    for i in range(abs(crab-quer)):
        fuel += i + 1

print(quer, fuel)
