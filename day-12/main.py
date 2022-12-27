import numpy as np
from functools import partial
import networkx as nx
from itertools import product
from tqdm import tqdm


# Solution components
@partial(np.frompyfunc, nin=1, nout=1)
def numeric_height(h):
    if h == 'S':
        return ord('z') - ord('a') + 1
    elif h == 'E':
        return ord('z') - ord('a')
    else:
        return ord(h) - ord('a')


def build_graph(m):
    G = nx.DiGraph()
    
    for i, j in product(range(m.shape[0]), range(m.shape[1])):
        G.add_node((i, j))

    for i, j in product(range(m.shape[0]), range(m.shape[1])):
        for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            ii = np.clip(i + di, 0, m.shape[0] - 1)
            jj = np.clip(j + dj, 0, m.shape[1] - 1)
            if m[ii, jj] - m[i, j] <= 1:
                G.add_edge((i, j), (ii, jj))

    return G

# Load input
with open('/workspaces/adventofcode-2022/day-12/input.txt', 'r') as fp:
    m = np.array([list(l.strip()) for l in fp.readlines()])

source = tuple(np.argwhere(m == 'S')[0])
target = tuple(np.argwhere(m == 'E')[0])

m = numeric_height(m).astype(float)

G = build_graph(m)

# Part 1 solution
print(nx.shortest_path_length(G, source, target))

# Part 2 solution
for depth, layer in enumerate(nx.bfs_layers(G.reverse(), target)):
    if any(map(lambda x: m[x[0], x[1]] == 0.0, layer)):
        print(depth)
        break
