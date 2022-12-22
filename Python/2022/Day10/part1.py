file = "input.txt"

def get_signal_strength(fileobj):
    x_before = 1
    x = 1
    cycle = 0
    values = [0 for _ in range(6)]
    v_i = 0
    capture_in = 20

    while (line := fileobj.readline().strip()):
        x_before = x

        if line == "noop":
            cycle += 1
            capture_in -= 1
        elif line.startswith("addx"):
            cycle += 2
            capture_in -= 2
            x += int(line.split(" ")[1])

        print(line, x, capture_in, cycle)

        if capture_in <= 0:
            if x_before == x: # when operation was noop
                values[v_i] = cycle * x
                capture_in = 40
            else:
                values[v_i] = (cycle) * x_before

                capture_in = 39
            print(values[v_i], x, x_before)
            v_i += 1

    return sum(values)


if __name__ == "__main__":
    with open(file, "r") as open_file:
        print(get_signal_strength(open_file))
