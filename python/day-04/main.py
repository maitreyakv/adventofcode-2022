import pandas as pd
import numpy as np
from functools import partial

# Load input
df = pd.read_csv(
    '/workspaces/adventofcode-2022/day-04/input.txt',
    header=None,
    names=['elf1', 'elf2']
)
df.elf1 = df.elf1.apply(lambda x: list(map(int, x.split('-'))))
df.elf2 = df.elf2.apply(lambda x: list(map(int, x.split('-'))))

# Part 1 solution
@partial(np.frompyfunc, nin=2, nout=1)
def contains(left, right):
    x = sorted(left + right)
    return x[1:3] in [left, right]

print(contains(df.elf1, df.elf2).sum())

# Part 2 solution
@partial(np.frompyfunc, nin=2, nout=1)
def overlaps(left, right):
    return not (left[0] > right[1] or right[0] > left[1])

print(overlaps(df.elf1, df.elf2).sum())