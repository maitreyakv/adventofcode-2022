from components import Item, Monkey, simulate

# See this link for Part 2 hint:
# https://chasingdings.com/2022/12/11/advent-of-code-day-11-monkey-in-the-middle/
for reduce_worry, rounds in zip(
    [lambda w: w // 3, lambda w: w % 9699690], 
    [20, 10_000]
):
    monkeys = [
        Monkey(
            init_items=[83, 88, 96, 79, 86, 88, 70],
            operation=lambda w: w * 5,
            test=lambda w: w % 11 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[59, 63, 98, 85, 68, 72],
            operation=lambda w: w * 11,
            test=lambda w: w % 5 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[90, 79, 97, 52, 90, 94, 71, 70],
            operation=lambda w: w + 2,
            test=lambda w: w % 19 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[97, 55, 62],
            operation=lambda w: w + 5,
            test=lambda w: w % 13 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[74, 54, 94, 76],
            operation=lambda w: w * w,
            test=lambda w: w % 7 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[58],
            operation=lambda w: w + 4,
            test=lambda w: w % 17 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[66, 63],
            operation=lambda w: w + 6,
            test=lambda w: w % 2 == 0,
            reduce_worry=reduce_worry
        ),
        Monkey(
            init_items=[56, 56, 90, 96, 68],
            operation=lambda w: w + 7,
            test=lambda w: w % 3 == 0,
            reduce_worry=reduce_worry
        ),
    ]

    monkeys[0].link_monkeys(monkeys[2], monkeys[3])
    monkeys[1].link_monkeys(monkeys[4], monkeys[0])
    monkeys[2].link_monkeys(monkeys[5], monkeys[6])
    monkeys[3].link_monkeys(monkeys[2], monkeys[6])
    monkeys[4].link_monkeys(monkeys[0], monkeys[3])
    monkeys[5].link_monkeys(monkeys[7], monkeys[1])
    monkeys[6].link_monkeys(monkeys[7], monkeys[5])
    monkeys[7].link_monkeys(monkeys[4], monkeys[1])

    simulate(monkeys, rounds, verbose=False)
    print()
