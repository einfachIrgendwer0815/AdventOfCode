file = "input.txt"

def find_marker(text, distincts):
    buf_size = distincts - 1
    double_counter = 0 # keeps track of how many new character are required to push out the latest double character

    for i in range(buf_size-1, -1, -1):
        if text[i] in text[i+1:buf_size]:
            double_counter = i+1
            break

    for index, l in enumerate(text[buf_size:], start=buf_size):
        if double_counter > 0:
            double_counter -= 1

        for i in range(buf_size):
            if text[index - buf_size + i] == l and double_counter < (i+1):
                double_counter = i + 1

        if double_counter == 0:
            return index + 1


if __name__ == "__main__":
    with open(file, "r") as open_file:
        line = open_file.readline().strip()

        print("Part 1:", find_marker(line, 4))
        print("Part 2:", find_marker(line, 14))
