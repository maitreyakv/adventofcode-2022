from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Sequence, Optional, Mapping
from collections import deque
from tqdm import tqdm


@dataclass
class Item:
    worry: int


class Monkey:
    def __init__(
        self, init_items: Sequence[int], operation: Callable, test: Callable,
        reduce_worry: Callable
    ) -> Monkey:
        self._items: deque[Item] = deque(Item(w) for w in init_items)
        self._operation: Callable = operation
        self._test: Callable = test
        self._reduce_worry: Callable = reduce_worry
        self._monkey_link: Optional[Mapping] = None
        self._n_inspect: int = 0

    def link_monkeys(self,  monkey_true: Monkey, monkey_false: Monkey) -> None:
        self._monkey_link = {
            True: monkey_true, 
            False: monkey_false
        }

    def receive_item(self, item) -> None:
        self._items.append(item)

    def turn(self):
        while self._items:
            item: Item = self._items.popleft()
            item.worry = self._operation(item.worry)
            item.worry = self._reduce_worry(item.worry)
            self._n_inspect += 1
            self._monkey_link[self._test(item.worry)].receive_item(item)

    @property
    def items_str(self) -> str:
        return ', '.join(str(item.worry) for item in self._items)


def simulate(monkeys, rounds, verbose=False) -> None:
    for round in tqdm(range(1, rounds+1)):
        for i, monkey in enumerate(monkeys):
            monkey.turn()
            
        if verbose:
            print(f'After round {round}:')
            for i, monkey in enumerate(monkeys):
                print(f'  Monkey {i}: {monkey.items_str}')
    
    for i, monkey in enumerate(monkeys):
        print(f'Monkey {i} inspected {monkey._n_inspect} items')