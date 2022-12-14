from components import Item, Monkey, simulate

monkeys = [
    Monkey(
        init_items=[79, 98],
        operation=lambda w: w * 19,
        test=lambda w: w % 23 == 0,
        reduce_worry=lambda w: w // 3
    ),
    Monkey(
        init_items=[54, 65, 75, 74],
        operation=lambda w: w + 6,
        test=lambda w: w % 19 == 0,
        reduce_worry=lambda w: w // 3
    ),
    Monkey(
        init_items=[79, 60, 97],
        operation=lambda w: w * w,
        test=lambda w: w % 13 == 0,
        reduce_worry=lambda w: w // 3
    ),
    Monkey(
        init_items=[74],
        operation=lambda w: w + 3,
        test=lambda w: w % 17 == 0,
        reduce_worry=lambda w: w // 3
    ),
]

monkeys[0].link_monkeys(monkeys[2], monkeys[3])
monkeys[1].link_monkeys(monkeys[2], monkeys[0])
monkeys[2].link_monkeys(monkeys[1], monkeys[3])
monkeys[3].link_monkeys(monkeys[0], monkeys[1])

simulate(monkeys, 20, verbose=True)
