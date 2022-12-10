file = "input.txt"

import re

cd_pattern = re.compile("\$ cd ([a-zA-Z]+|/|\.\.)")

class Dir():
    def __init__(self, name, parent, root=False):
        self.name = name
        self.files = {}
        self.dirs = {}
        self.size = 0
        self.parent = parent
        self.is_root = root

    def add_to_size(self, add):
        self.size += add

        if not self.is_root:
            self.parent.add_to_size(add)

class File():
    def __init__(self, name, size):
        self.name = name
        self.size = size


def build_tree(file_obj):
    root = None

    cwd = None

    while (line := file_obj.readline().strip()):
        if match := cd_pattern.match(line):
            if match.group(1) == "/":
                if root == None:
                    root = Dir("/", None, True)
                cwd = root
            elif match.group(1) == "..":
                cwd = cwd.parent
            else:
                cwd = cwd.dirs[match.group(1)]

        elif not line.startswith("$"):
            if splitted := line.split(" "):
                if splitted[0] == "dir":
                    if splitted[1] not in cwd.dirs.keys():
                        cwd.dirs[splitted[1]] = Dir(splitted[1], cwd)

                else:
                    if splitted[1] not in cwd.files.keys():
                        size = int(splitted[0])
                        cwd.files[splitted[1]] = File(splitted[1], size)
                        cwd.add_to_size(size)
    return root

def calc_size_of_small_dirs(root, cap):
    total = 0

    def inner(dir):
        nonlocal total

        if dir.size <= cap:
            total += dir.size

        for d in dir.dirs.values():
            inner(d)

    inner(root)
    return total


def calc_total_deleted(root, total_disk_space, update_size):
    unused_space = total_disk_space - root.size
    left_required = update_size - unused_space

    total_deleted = root.size

    def inner(dir):
        nonlocal total_deleted

        if dir.size >= left_required and dir.size < total_deleted:
            total_deleted = dir.size

        for d in dir.dirs.values():
            inner(d)

    inner(root)
    return total_deleted


if __name__ == "__main__":
    with open(file, "r") as open_file:
        tree = build_tree(open_file)
        print("Part 1:", calc_size_of_small_dirs(tree, 100000))
        print("Part 2:", calc_total_deleted(tree, 70000000, 30000000))
