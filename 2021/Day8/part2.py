import itertools

def containsAllOfList(lista, listb):
    for i in lista:
        if i not in listb:
            return False

    return True

file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        split = [list(i) for _, i in itertools.groupby(line.strip().split(), key='|'.__ne__)]
        split.pop(1)
        lines.append(split)

#lines = [[['acedgfb', 'cdfbe', 'gcdfa', 'fbcad', 'dab', 'cefabd', 'cdfgeb', 'eafb', 'cagedb', 'ab'], ['cdfeb', 'fcadb', 'cdfeb', 'cdbaf']]]
counter = 0

for line in lines:
    digits = [None for _ in range(10)]
    wirepos = [None for _ in range(7)]

    for i in line[0]:
        if (length := len(i)) == 2:
            digits[1] = i
        elif length == 3:
            digits[7] = i
        elif length == 4:
            digits[4] = i
        elif length == 7:
            digits[8] = i

    for i in digits[7]:
        if i not in digits[1]:
            wirepos[0] = i
            break

    for i in line[0]:
        if  len(i) == 5 and containsAllOfList(digits[7], i): # must be a 3
            digits[3] = i

            for j in i:
                if j in digits[4] and j not in digits[7]:
                    wirepos[6] = j

                elif j not in digits[4] and j not in digits[7]:
                    wirepos[3] = j

            break

    for i in line[0]:
        if len(i) == 5 and not containsAllOfList(digits[7], i):
            partsOf4 = 0
            for j in i:
                if j in digits[4]:
                    partsOf4 += 1

            if partsOf4 == 3: # must be a 5
                digits[5] = i

                fiveCopy = list(i).copy()
                fiveCopy.remove(wirepos[0])
                fiveCopy.remove(wirepos[3])
                fiveCopy.remove(wirepos[6])

                for j in fiveCopy:
                    if j in digits[1]:
                        wirepos[2] = j
                    else:
                        wirepos[5] = j

            elif partsOf4 == 2: # must be a 2
                digits[2] = i

                twoCopy = list(i).copy()
                twoCopy.remove(wirepos[0])
                twoCopy.remove(wirepos[3])
                twoCopy.remove(wirepos[6])

                for j in twoCopy:
                    if j in digits[1]:
                        wirepos[1] = j
                    else:
                        wirepos[4] = j

    for i in line[0]:
        if len(i) == 6:
            if wirepos[6] not in i:
                digits[0] = i

            elif wirepos[1] not in i:
                digits[6] = i

            else:
                digits[9] = i

    number = ""
    for i in line[1]:
        for index, j in enumerate(digits):
            if len(i) == len(j) and containsAllOfList(i,j):
                number += str(index)

    counter += int(number)

print(counter)
