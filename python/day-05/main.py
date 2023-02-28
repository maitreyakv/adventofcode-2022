import pandas as pd
import numpy as np
from collections import deque


class Crane:
    def __init__(self, n_stacks, new_claw=False):
        self._stack = {i: deque() for i in range(1, n_stacks+1)}
        self._claw = deque()
        self._new_claw = new_claw

    def perform_instruction(self, n, s, d):
        pop_method = self._claw.pop if self._new_claw else self._claw.popleft
        for _ in range(n):
            self._claw.append(self._stack[s].pop())
        for _ in range(n):
            self._stack[d].append(pop_method())
        
    def _init_stack(self, id_, items):
        for item in filter(lambda x: x != ' ', items):
            self._stack[id_].append(item)

    def top(self, id_):
        return self._stack[id_][-1]

    def __len__(self):
        return len(self._stack)

    @classmethod
    def from_lines(cls, lines, new_claw):
        arr = np.array([list(l.strip('\n')) for l in lines])
        mask = arr[-1, :] != ' '
        arr = arr[:, mask]
        
        crane = Crane(arr.shape[-1], new_claw)
        for stack in arr.T:
            crane._init_stack(int(stack[-1]), stack[-2::-1])

        return crane        

def simulate(lines, new_claw=False):
    crane = Crane.from_lines(
        [l for l in lines if not l.startswith('move')][:-1], 
        new_claw=new_claw
    )

    for line in [l for l in lines if l.startswith('move')]:
        line = line.strip().split(' ')
        crane.perform_instruction(
            n=int(line[1]),
            s=int(line[3]),
            d=int(line[5]),
        )

    print(''.join(crane.top(i) for i in range(1, len(crane)+1)))

# Part 1 solution
with open('/workspaces/adventofcode-2022/day-05/input.txt', 'r') as fp:
    simulate(fp.readlines())

# Part 2 solution
with open('/workspaces/adventofcode-2022/day-05/input.txt', 'r') as fp:
    simulate(fp.readlines(), new_claw=True)