import string

file = "input.txt"
LETTERS = {letter: i+1 for i, letter in enumerate(string.ascii_letters)}


def find_shared_item(content):
    middle = len(content) // 2
    left = content[:middle]
    right = content[middle:]

    for item in left:
        if item in right:
            return item


if __name__ == "__main__":
    total = 0

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            item = find_shared_item(line.strip())
            total += LETTERS[item]

    print(total)
