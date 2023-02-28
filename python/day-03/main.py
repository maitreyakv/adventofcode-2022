import pandas as pd
import numpy as np
from functools import partial

# Load input
df = pd.read_csv(
    '/workspaces/adventofcode-2022/day-03/input.txt',
    header=None,
    names=['rucksack']
)

# Part 1 solution
@partial(np.frompyfunc, nin=1, nout=2)
def split(rucksack):
    n = len(rucksack)
    left, right = rucksack[:n//2], rucksack[n//2:]
    return left, right

@partial(np.frompyfunc, nin=2, nout=1)
def common(left, right):
    return list(set(left).intersection(right))[0]

@partial(np.frompyfunc, nin=1, nout=1)
def priority(x):
    if x.lower() == x:
        return ord(x) - ord('a') + 1
    else:
        return ord(x) - ord('A') + 27

print(priority(common(*split(df.rucksack))).sum())

# Part 2 solution
df = ((
    df
    .groupby(df.index // 3)
    .agg({'rucksack': lambda x: list(x)}))
)

@partial(np.frompyfunc, nin=1, nout=1)
def badge(group):
    return list(set.intersection(*(set(x) for x in group)))[0]

print(priority(badge(df.rucksack)).sum())