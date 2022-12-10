file = "input.txt"

def find_marker(text, distincts):
    buf_size = distincts - 1
    buffer = ['' for _ in range(buf_size)]
    double_counter = 0 # keeps track of how many new character are required to push out the latest double character

    letters = 0

    for l in text:
        letters += 1
        if double_counter > 0:
            double_counter -= 1

        for index, item in enumerate(buffer):
            if item == l and double_counter < (index+1):
                double_counter = index + 1

        if double_counter == 0 and letters >= distincts:
            return letters

        buffer.pop(0)
        buffer.append(l)


if __name__ == "__main__":
    with open(file, "r") as open_file:
        line = open_file.readline().strip()

        print("Part 1:", find_marker(line, 4))
        print("Part 2:", find_marker(line, 14))
