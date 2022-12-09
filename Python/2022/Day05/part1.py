file = "input.txt"

def setup_stacks(stack_lines, stacks=9):
    stacks = [[] for _ in range(9)]

    for line in stack_lines:
        for stack_index, line_index in enumerate(range(1, len(line), 4)):
            if line[line_index] != " " and line[line_index] != "\n":
                stacks[stack_index].append(line[line_index])

    for stack in stacks:
        stack.reverse()

    return stacks

if __name__ == "__main__":
    stack_lines = []
    stacking = True

    stacks = None

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            if stacking:
                stack_lines.append(line)
                if line.strip() == "":
                    stacking = False
                    stacks = setup_stacks(stack_lines)

                continue

            move = list(map(int, line.strip().split(" ")[1::2]))

            containers = [stacks[move[1] - 1].pop() for _ in range(move[0])]
            stacks[move[2] - 1] += containers

    print(''.join(stack[-1] for stack in stacks))
