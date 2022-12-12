file = "input.txt"

def find_highest_scenic_score(grid):
    high_score = 0

    height = len(grid)
    width = len(grid[0])

    for y in range(height):
        for x in range(width):
            left_range = 0
            for x_2 in range(x-1, -1, -1):
                left_range += 1
                if grid[y][x_2] >= grid[y][x]:
                    break

            right_range = 0
            for x_2 in range(x+1, width):
                right_range += 1
                if grid[y][x_2] >= grid[y][x]:
                    break

            up_range = 0
            for y_2 in range(y-1, -1, -1):
                up_range += 1
                if grid[y_2][x] >= grid[y][x]:
                    break

            down_range = 0
            for y_2 in range(y+1, height):
                down_range += 1
                if grid[y_2][x] >= grid[y][x]:
                    break

            score = left_range * right_range * up_range * down_range
            high_score = max(high_score, score)

    return high_score


if __name__ == "__main__":
    lines = []

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            lines.append(list(map(int, line.strip())))

    print(find_highest_scenic_score(lines))
