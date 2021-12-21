import statistics

file = "input.txt"

crabsPos = []
with open(file, "r") as open_file:
    line = open_file.readline()
    crabsPos = [ i for i in map(int, line.strip().split(',')) ]

median = int(statistics.median(crabsPos))

fuel = 0
for crab in crabsPos:
    fuel += abs(crab - median)

print(fuel)
