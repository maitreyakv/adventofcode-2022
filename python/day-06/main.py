import pandas as pd

# Load input
with open('/workspaces/adventofcode-2022/day-06/input.txt', 'r') as fp:
    df = pd.DataFrame({'buffer': list(fp.read().strip())})

# Part 1 solution
print((
    df
    .buffer
    .apply(lambda x: ord(x))
    .rolling(4)
    .apply(lambda x: len(x.unique()) == 4).idxmax()
    +1
))

# Part 2 solution
print((
    df
    .buffer
    .apply(lambda x: ord(x))
    .rolling(14)
    .apply(lambda x: len(x.unique()) == 14).idxmax()
    +1
))

