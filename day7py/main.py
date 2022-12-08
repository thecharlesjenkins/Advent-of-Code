import re

class Node:
    def __init__(self, children: dict[str,"Node"], parent, size, name) -> None:
        self.children = children
        self.parent = parent
        self.size = size
        self.name = name


def calculate_size(curr: Node, count: list[int]):
    if len(curr.children) != 0:
        for child in curr.children.values():
            curr.size += calculate_size(child, count)

    if curr.size < 100000:
        count[0] += curr.size
    return curr.size


def find_directory(curr: Node, delete_size: int):
    size = -1
    if len(curr.children) != 0:
        minimum = []
        for child in curr.children.values():
            child_response = find_directory(child, delete_size)
            curr.size += child_response[0]
            minimum.append(child_response[1])
        minimum.append(curr.size)
        skimmed = list(filter(lambda el: el >= delete_size, minimum))
        if skimmed:
            size = skimmed[0]

    return (curr.size, size)

cd = re.compile(r"\$ cd (\w+)")
cd2 = re.compile(r"\$ cd \.\.")
ls = re.compile(r"\$ ls")
dir = re.compile(r"dir (\w+)")
file = re.compile(r"(\d+) (\w+)")


root = Node({},None,0,"root")
current = root
cdcount = 0
linecount = 0
with open("input.txt") as f:
    f.readline()
    for line in f:
        linecount += 1
        cdmatch = cd.match(line)
        cd2match = cd2.match(line)
        lsmatch = ls.match(line)
        dirmatch = dir.match(line)
        filematch = file.match(line)
        if dirmatch:
            if dirmatch.groups()[0] not in current.children:
                current.children[dirmatch.groups()[0]] = Node({},current,0,dirmatch.groups()[0])
            else:
                print('already exists')
        elif lsmatch:
            continue
        elif cdmatch:
            if cdmatch.groups()[0] in current.children:
                next = current.children[cdmatch.groups()[0]]
            else:
                next = Node({},current,0,cdmatch.groups()[0])
            current = next
            cdcount += 1
        elif cd2match:
            current = current.parent
            cdcount += 1
        elif filematch:
            current.size += int(filematch.groups()[0])
        else:
            print("no match!!!")

totalsize = [0]
# calculate_size(root, totalsize) part 1

unused_size = 70000000 - 48748071
delete_size = 30000000 - unused_size
print(delete_size)
print(find_directory(root, delete_size))
