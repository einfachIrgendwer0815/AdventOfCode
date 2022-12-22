file = "input.txt"

def render_image(fileobj):
    current_operation = ""
    timeToFinish = 0
    x = 1
    crt_rows = [["" for _ in range(40)] for _ in range(6)]
    current_row = 0
    current_column = 0

    run = True
    while run:
        # start of cycle
        if current_operation == "":
            if (line := next_line_or_exit(fileobj)) != None:
                current_operation = line
                if current_operation == "noop":
                    timeToFinish = 1
                else:
                    timeToFinish = 2
            else:
                break

        # drawing on the screen during the cycle
        if x-1 <= current_column <= x+1:
            crt_rows[current_row][current_column] = "#"
        else:
            crt_rows[current_row][current_column] = "."

        # end of cycle
        current_column += 1
        if current_column >= 40:
            current_row += 1
            current_column = 0

        timeToFinish -= 1
        if timeToFinish == 0:
            if current_operation.startswith("addx"):
                x += int(current_operation.split(" ")[1])
            current_operation = ""

    return crt_rows


def next_line_or_exit(fileobj):
    if not (line := fileobj.readline().strip()):
        return None
    else:
        return line


if __name__ == "__main__":
    with open(file, "r") as open_file:
        for l in render_image(open_file):
            print(''.join(l))
