file = "input.txt"

def count_visible_trees(grid):
    height = len(grid)
    width = len(grid[0])

    for y in range(height):
        for x in range(width):
            grid[y][x] = [grid[y][x], False]

    up_highest = [-1 for _ in range(width)]
    for y in range(height):
        left_highest = -1

        for x in range(width):
            if grid[y][x][0] > left_highest:
                left_highest = grid[y][x][0]
                grid[y][x][1] = True

            if grid[y][x][0] > up_highest[x]:
                up_highest[x] = grid[y][x][0]
                grid[y][x][1] = True

        right_highest = -1
        for x in range(width - 1, -1, -1):
            if grid[y][x][0] > right_highest:
                right_highest = grid[y][x][0]
                grid[y][x][1] = True

    count = 0

    down_highest = [-1 for _ in range(width)]
    for y in range(height - 1, -1, -1):
        for x in range(width):
            if grid[y][x][0] > down_highest[x]:
                down_highest[x] = grid[y][x][0]
                grid[y][x][1] = True

            count += grid[y][x][1]

    return count


if __name__ == "__main__":
    lines = []

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            lines.append(list(map(int, line.strip())))

    print(count_visible_trees(lines))
