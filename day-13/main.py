from ast import literal_eval
from functools import cmp_to_key


# Solution components
def in_order(l, r):
    if isinstance(l, int) and isinstance(r, int):
        if l < r:
            return 1
        elif l == r:
            return 0
        else:
            return -1
    elif isinstance(l, list) and isinstance(r, list):
        for lv, rv in zip(l, r):
            res = in_order(lv, rv)
            if res == 1:
                return 1
            elif res == 0:
                continue
            elif res == -1:
                return -1
        if len(l) < len(r):
            return 1
        elif len(l) == len(r):
            return 0
        else:
            return -1
    elif isinstance(l, list) and not isinstance(r, list):
        return in_order(l, [r])
    elif not isinstance(l, list) and isinstance(r, list):
        return in_order([l], r)


# Load input
l_list = []
r_list = []
with open('/workspaces/adventofcode-2022/day-13/input.txt', 'r') as fp:
    lines = fp.readlines()
    for i in range(len(lines) // 3 + 1):
        l_list.append(literal_eval(lines[3*i].strip()))
        r_list.append(literal_eval(lines[3*i+1].strip()))


# Part 1 solution
idx_sum = 0
for i, (l, r) in enumerate(zip(l_list, r_list), start=1):
    if in_order(l, r) == 1:
        idx_sum += i

print(idx_sum)

# Part 2 solution
idx_prod = 1
packets = l_list + r_list + [[[2]], [[6]]]
packets = sorted(packets, key=cmp_to_key(in_order), reverse=True)
for i, packet in enumerate(packets, start=1):
    if packet == [[2]] or packet == [[6]]:
        idx_prod *= i

print(idx_prod)