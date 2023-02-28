import pandas as pd
import numpy as np

# Solution components
class Device:
    def __init__(self):
        self._x = 1
        self._cycle_count = 1
        self._history = []
        self._crt = []

    def _cycle(self):
        if self._x - 1 <= (self._cycle_count - 1) % 40 <= self._x + 1:
            self._crt.append('#')
        else:
            self._crt.append('.')
        self._history.append({'cycle': self._cycle_count, 'x': self._x})
        self._cycle_count += 1

    def _op_add(self, val):
        self._cycle()
        self._cycle()
        self._x += val

    def process_op(self, op):
        op = op.strip().split(' ')
        if op[0] == 'addx':
            self._op_add(int(op[1]))
        elif op[0] == 'noop':
            self._cycle()

    @property
    def history(self):
        df = pd.DataFrame(self._history)
        df.set_index('cycle', drop=True, inplace=True)
        df['signal_strength'] = df.index * df.x
        return df

    @property
    def crt(self):
        return self._crt


# Part 1 solution
device = Device()
with open('/workspaces/adventofcode-2022/day-10/input.txt', 'r') as fp:
    for op in fp.readlines():
        device.process_op(op)

idx = np.arange(20, len(device.history)+1, 40)
print(device.history.loc[idx].signal_strength.sum())

# Part 2 solution
for line in np.array(device.crt).reshape(-1, 40):
    print(''.join(line))