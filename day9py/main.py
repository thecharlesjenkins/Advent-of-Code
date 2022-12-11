import re

# positions visited based off of the starting point
visited = set()
visited.add((0,0))

move = re.compile(r"([UDLR]) (\d+)")

with open("input.txt") as f:
    tailcol = 0
    tailrow = 0

    headcol = 0
    headrow = 0
    for line in f:
        match = move.match(line)
        if match != None:
            for number in range(int(match.groups()[1])):
                if match.groups()[0] == 'U':
                    headrow -= 1
                elif match.groups()[0] == 'D':
                    headrow += 1
                elif match.groups()[0] == 'R':
                    headcol += 1
                elif match.groups()[0] == 'L':
                    headcol -= 1

                if tailcol == headcol: # in the same column
                    if tailrow < headrow - 1:
                        # move one up
                        tailrow += 1
                    elif tailrow > headrow + 1:
                        # move one down
                        tailrow -= 1
                elif tailrow == headrow: # in the same row
                    if tailcol < headcol - 1:
                        # move one to the right
                        tailcol += 1
                    elif tailcol > headcol + 1:
                        # move one to  the left
                        tailcol -= 1
                else: # needs to move along diagonal
                    # check for not exactly diagonal
                    exact_diagonal = (tailcol == headcol - 1 and tailrow == headrow - 1) or (tailcol == headcol + 1 and tailrow == headrow + 1) or (tailcol == headcol + 1 and tailrow == headrow - 1) or (tailcol == headcol - 1 and tailrow == headrow + 1)

                    if not exact_diagonal:
                        if tailcol < headcol: # somewhere to the left
                            if tailrow < headrow: # diagonal left down
                                tailrow += 1
                                tailcol += 1
                            else: # diagonal left up
                                tailrow -= 1
                                tailcol += 1
                        else: # somewhere right
                            if tailrow < headrow: # diagonal right down
                                tailrow += 1
                                tailcol -= 1
                            else: # diagonal right up
                                tailrow -= 1
                                tailcol -= 1
        
                visited.add((tailrow, tailcol))
                print((tailrow, tailcol))

print(visited)
print(len(visited))
