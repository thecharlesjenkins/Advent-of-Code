import re

# positions visited based off of the starting point
visited = set()
visited.add((0,0))

move = re.compile(r"([UDLR]) (\d+)")

num_knots = 9

knots = []

for i in range(num_knots+1):
    knots.append([0,0])

with open("input.txt") as f:
    for line in f:
        match = move.match(line)
        if match != None:
            for number in range(int(match.groups()[1])):
                for index, knot in enumerate(knots):
                    if index == 0:
                        print('head')
                        if match.groups()[0] == 'U':
                            knots[0][0] -= 1
                        elif match.groups()[0] == 'D':
                            knots[0][0] += 1
                        elif match.groups()[0] == 'R':
                            knots[0][1] += 1
                        elif match.groups()[0] == 'L':
                            knots[0][1] -= 1
                    else:
                        tailrow = knot[0]
                        tailcol = knot[1]
                        headrow = knots[index-1][0]
                        headcol = knots[index-1][1]
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
                        knot[0] = tailrow
                        knot[1] = tailcol
                    if index == num_knots:
                        print((knots[index][0], knots[index][1]))
                        visited.add((knots[index][0], knots[index][1]))

print(visited)
print(len(visited))
