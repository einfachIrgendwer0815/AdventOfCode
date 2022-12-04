import string

file = "input.txt"
LETTERS = {letter: i+1 for i, letter in enumerate(string.ascii_letters)}


def find_shared_item(r1, r2, r3):
    for item in r1:
        if item in r2 and item in r3:
            return item


if __name__ == "__main__":
    total = 0

    with open(file, "r") as open_file:
        counter = 0
        rucksacks = ["","",""]

        while (line := open_file.readline()):
            rucksacks[counter] = line.strip()
            counter += 1

            if counter > 2:
                counter = 0

                item = find_shared_item(*rucksacks)
                total += LETTERS[item]

    print(total)
