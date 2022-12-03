file = "input.txt"

SHAPE_POINTS = {
    "A": 1,
    "B": 2,
    "C": 3,
}

OUTCOME_POINTS = {
    "X": 0,
    "Y": 3,
    "Z": 6,
}

DEFEATS = {
    "A": "C",
    "B": "A",
    "C": "B",
}

def calc_points(opponent, end):
    points = OUTCOME_POINTS[end]

    if end == "Y":
        points += SHAPE_POINTS[opponent]
    elif end == "X":
        points += SHAPE_POINTS[DEFEATS[opponent]]
    elif end == "Z":
        for shape in DEFEATS.keys():
            if DEFEATS[shape] == opponent:
                points += SHAPE_POINTS[shape]
                break

    return points

if __name__ == "__main__":
    score = 0

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            opponent, end = line.strip().split(" ")
            score += calc_points(opponent, end)

    print("My score is:", score)
