import numpy as np

# Load input
with open('/workspaces/adventofcode-2022/day-09/input.txt', 'r') as fp:
    lines = fp.readlines()

# Solution components
class Rope:
    def __init__(self, length):
        self.knots = [np.zeros(2) for _ in range(length)]
        self.tail_history = set()

    def move(self, step):
        self.knots[0] += step
        for i in range(1, len(self.knots)):
            if np.amax(np.abs(self.knots[i-1] - self.knots[i])) > 1:
                self.knots[i] += np.clip(self.knots[i-1] - self.knots[i], -1, 1)
        self.tail_history.add(tuple(self.knots[-1]))

def simulate(lines, length):
    step_map = {
        'R': np.array([0, 1]),
        'L': np.array([0, -1]),
        'U': np.array([-1, 0]),
        'D': np.array([1, 0]),
    }
    rope = Rope(length)
    for line in lines:
        direction, number = line.strip().split(' ')
        step = step_map[direction]
        for _ in range(int(number)):
            rope.move(step)
    print(len(rope.tail_history))

# Part 1 solution
simulate(lines, length=2)

# Part 2 solution
simulate(lines, length=10)