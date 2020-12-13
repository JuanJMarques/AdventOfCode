def connect_adapters_rec(adapters: list, curr_joltage: int, objective_joltage: int, history: dict) -> object:
    if curr_joltage in history.keys():
        return history[curr_joltage]
    if curr_joltage + 3 == objective_joltage:
        history[curr_joltage] = 1
        return 1
    elif curr_joltage + 3 > objective_joltage:
        history[curr_joltage] = 0
        return 0
    connect_1 = 0
    if curr_joltage + 1 in adapters:
        connect_1 = connect_adapters_rec(adapters, curr_joltage + 1, objective_joltage, history)
    connect_2 = 0
    if curr_joltage + 2 in adapters:
        connect_2 = connect_adapters_rec(adapters, curr_joltage + 2, objective_joltage, history)
    connect_3 = 0
    if curr_joltage + 3 in adapters:
        connect_3 = connect_adapters_rec(adapters, curr_joltage + 3, objective_joltage, history)
    connect_ways = connect_1 + connect_2 + connect_3
    history[curr_joltage] = connect_ways
    return connect_ways


def connect_adapters(adapters):
    min_joltage = 0
    device_adapter = max(adapters) + 3
    history = dict()
    return connect_adapters_rec(adapters, min_joltage, device_adapter, history)


adapters = []
with open('input.txt') as lines:
    for line in lines:
        if '' != line.strip():
            in_number = int(line.strip())
            adapters.append(in_number)
number_of_ways_to_connect = connect_adapters(adapters)
print('the number of ways to connect the adapters is: {}'.format(number_of_ways_to_connect))
