file = "input.txt"

EQUIVALENTS = {
    "X": "A",
    "Y": "B",
    "Z": "C",
}

SHAPE_POINTS = {
    "A": 1,
    "B": 2,
    "C": 3,
}

OUTCOME_POINTS = {
    "lose": 0,
    "draw": 3,
    "win": 6,
}

DEFEATS = {
    "A": "C",
    "B": "A",
    "C": "B",
}

def calc_points(opponent, me):
    me = EQUIVALENTS[me]

    points = SHAPE_POINTS[me]

    if opponent == me:
        points += OUTCOME_POINTS["draw"]
    elif DEFEATS[opponent] == me:
        points += OUTCOME_POINTS["lose"]
    else:
        points += OUTCOME_POINTS["win"]

    return points

if __name__ == "__main__":
    score = 0

    with open(file, "r") as open_file:
        while (line := open_file.readline()):
            opponent, me = line.strip().split(" ")
            score += calc_points(opponent, me)

    print("My score is:", score)
