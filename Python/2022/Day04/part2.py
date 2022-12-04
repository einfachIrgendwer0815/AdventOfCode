import re

file = "input.txt"
pattern = re.compile(r"(\d*)-(\d*),(\d*)-(\d*)")


def overlaps(r1, r2):
    if r1[0] <= r2[0] and r1[1] >= r2[0]:
        return True

    return False

if __name__ == "__main__":
    count = 0

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            match = pattern.match(line.strip())
            numbers = [int(i) for i in match.group(1,2,3,4)]
            pair1 = numbers[:2]
            pair2 = numbers[2:]

            count += overlaps(pair1, pair2) or overlaps(pair2, pair1)

    print(count)
