import numpy as np

# Load input
with open('/workspaces/adventofcode-2022/day-08/input.txt', 'r') as fp:
    t = np.array([list(l.strip()) for l in fp.readlines()]).astype(int)

# Part 1 solution
ms = np.zeros(t.shape + (4,))
for axis in range(2):
    for direction in range(2):
        x = np.pad(t, 2, constant_values=-1)
        if direction: 
            x = np.flip(x, axis=axis)
        x = np.maximum.accumulate(x, axis=axis)
        if direction: 
            x = np.flip(x, axis=axis)
        x = np.roll(x, shift=(-1)**direction, axis=axis)
        ms[:, :, 2*axis+direction] = x[2:-2, 2:-2]

print(
    np.sum(np.logical_or.reduce(t[..., None] > ms, axis=2))
)

# Part 2 solution
scores = np.ones_like(t, dtype=float)
for axis in range(2):
    for direction in range(2):
        v = np.ones_like(t) * np.inf
        for r in range(1, t.shape[axis]):
            x = np.roll(t, r*(-1)**direction, axis=axis)
            mask = x >= t
            v[mask] = np.minimum(v, r)[mask]
        depth = np.arange(t.shape[axis])[::(-1)**direction]
        depth = np.meshgrid(depth, depth)[(axis + 1) % 2]
        scores *= np.minimum(v, depth)

print(np.amax(scores))
