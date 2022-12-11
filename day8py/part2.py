score = 0

grid = []
with open('input.txt') as file:
    # place into an array
    for line in file:
        row = []
        for char in line:
            if char != '\n':
                row.append(int(char))
        grid.append(row)


# with open('results.txt', 'w') as f:
# iterate through all trees
for row in range(len(grid)):
    for col in range(len(grid[row])):
        # calculate metric for tree at (row, col)

        curr_tree_height = grid[row][col]
        right_distance = 0
        left_distance = 0
        down_distance = 0
        up_distance = 0
        # right distance
        for right in range(col+1, len(grid[row])):
            right_distance += 1
            if grid[row][right] >= curr_tree_height:
                break
        # left distance
        for left in range(col-1,-1,-1):
            left_distance += 1
            if grid[row][left] >= curr_tree_height:
                break
        # down distance
        for down in range(row+1, len(grid)):
            down_distance += 1
            if grid[down][col] >= curr_tree_height:
                break
        # up distance
        for up in range(row-1, -1, -1):
            up_distance += 1
            if grid[up][col] >= curr_tree_height:
                break
        
        score = max(right_distance * left_distance * down_distance * up_distance, score)
        
print(score)
