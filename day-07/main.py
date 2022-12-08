from __future__ import annotations

from dataclasses import dataclass, field
import pandas as pd
from typing import Mapping


@dataclass
class File:
    name: str
    size: int


@dataclass
class Directory:
    name: str
    parent: Directory = None
    contents: Mapping = field(init=False)

    def __post_init__(self):
        self.contents = {}

    def add(self, item):
        self.contents[item.name] = item

    @property
    def size(self):
        return sum((item.size for item in self.contents.values()))


df = pd.read_table(
    '/workspaces/adventofcode-2022/day-07/input.txt',
    header=None,
    names=['lines']
)

df['cmd_num'] = df.lines.str.startswith('$').cumsum()

root = Directory('/')

blocks = df.groupby('cmd_num')
cwd = None
all_items = []

for cmd_num, block in blocks:
    cmd = block.lines.iloc[0]
    if cmd.startswith('$ cd'):
        if cmd == '$ cd /':
            cwd = root
        elif cmd == '$ cd ..':
            cwd = cwd.parent
        else:
            cwd = cwd.contents[cmd.split(' ')[2]]
    elif cmd.startswith('$ ls'):
        for item in list(block.lines.iloc[1:]):
            size, name = item.split(' ')
            new_item = (
                Directory(name, cwd) 
                if item.startswith('dir') 
                else File(name, int(size))
            )
            cwd.add(new_item)
            all_items.append(new_item)


# Part 1 Solution
print(sum([
    item.size
    for item 
    in all_items 
    if isinstance(item, Directory) and item.size <= 100_000
]))


# Part 2 solution
required_delete_space = 30_000_000 - (70_000_000 - root.size)
print(sorted([
    item.size
    for item
    in all_items
    if isinstance(item, Directory) and item.size >= required_delete_space
])[0])