def findAllPaths(nodes):
    paths = []

    def _inner(path, node):
        nonlocal paths

        for branch in nodes[node]:
            if branch == "end":
                paths.append(path.copy())

            elif branch.lower() == branch:
                if branch in path:
                    continue
                else:
                    cpath = path.copy()
                    cpath.append(branch)
                    _inner(cpath, branch)

            else:
                cpath = path.copy()
                cpath.append(branch)
                _inner(cpath, branch)

    _inner([], "start")

    return paths

file = "input.txt"

lines = []

with open(file, "r") as open_file:
    while (line := open_file.readline()):
        lines.append(line.strip().split('-'))

nodes = {}

for line in lines:
    if line[0] not in nodes.keys():
        if line[1] != "start":
            nodes[line[0]] = [line[1]]
    else:
        if line[1] != "start":
            nodes[line[0]].append(line[1])

    if line[1] not in nodes.keys():
        if line[0] != "start":
            nodes[line[1]] = [line[0]]
    else:
        if line[0] != "start":
            nodes[line[1]].append(line[0])

if "end" in nodes.keys():
    del nodes["end"]

paths = findAllPaths(nodes)

print(len(paths))
