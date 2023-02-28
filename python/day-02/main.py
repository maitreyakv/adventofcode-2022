import numpy as np
import pandas as pd
from functools import partial


# Load data
df = pd.read_csv(
    '/workspaces/adventofcode-2022/day-02/input.txt',
    delimiter=' ',
    header=None,
    names=['other', 'mine']
)

# Part 1 Solution
@partial(np.frompyfunc, nin=2, nout=1)
def score(other, mine):
    _map = {
        'A': 0, 'B': 1, 'C': 2,
        'X': 0, 'Y': 1, 'Z': 2,
        0: 3, 1: 6, 2: 0
    }
    other = _map[other]
    mine = _map[mine]
    score_outcome = _map[(mine - other) % 3]
    score_shape = mine + 1
    return score_outcome + score_shape

print(score(df.other, df.mine).sum())

# Part 2 solution
df.rename(columns={'mine': 'outcome'}, inplace=True)

@partial(np.frompyfunc, nin=2, nout=1)
def choose(other, outcome):
    _map = {
        'A': 0, 'B': 1, 'C': 2,
        'X': -1, 'Y': 0, 'Z': 1,
        0: 'X', 1: 'Y', 2: 'Z'
    }
    other = _map[other]
    outcome = _map[outcome]
    mine = _map[(other + outcome) % 3]
    return mine

df['mine'] = choose(df.other, df.outcome)
print(score(df.other, df.mine).sum())