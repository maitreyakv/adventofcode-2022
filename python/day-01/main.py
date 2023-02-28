import pandas as pd

# Load data
df = pd.read_table(
    '/workspaces/adventofcode-2022/day-01/input.txt',
    skip_blank_lines=False,
    header=None,
    names=['calories']
)
df['id'] = df.isna().cumsum()

# Part 1 solution
print(df.groupby('id').sum().max())

# Part 2 solution
print((
    df
    .groupby('id')
    .sum()
    .sort_values('calories', ascending=False))
    .iloc[:3]
    .sum()
)