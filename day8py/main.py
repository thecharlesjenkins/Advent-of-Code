accessible_coords = set()


grid = []
with open('input.txt') as file:
    # place into an array
    for line in file:
        row = []
        for char in line:
            if char != '\n':
                row.append(int(char))
        grid.append(row)

# find all visible from left
for row in range(len(grid)):
    tallest = -1
    for col in range(len(grid[row])):
        if grid[row][col] > tallest:
            accessible_coords.add((row,col))
        tallest = max(grid[row][col], tallest)

# find all visible from right
for row in range(len(grid)):
    tallest = -1
    for col in range(len(grid[row])-1,-1,-1):
        if grid[row][col] > tallest:
            accessible_coords.add((row,col))
        tallest = max(grid[row][col], tallest)

# find all visible from top
for col in range(len(grid[0])):
    tallest = -1
    for row in range(len(grid)):
        if grid[row][col] > tallest:
            accessible_coords.add((row,col))

        tallest = max(grid[row][col], tallest)

# find all visible from bottom
for col in range(len(grid[0])):
    tallest = -1
    for row in range(len(grid)-1,-1,-1):
        if grid[row][col] > tallest:
            accessible_coords.add((row,col))

        tallest = max(grid[row][col], tallest)


print(len(accessible_coords))