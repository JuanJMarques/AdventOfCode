def parse_line(line: str) -> list:
    return [symbol == '#' for symbol in line]


def get_alive_neighbors(state: list, i: int, j: int, k: int) -> int:
    alive_neighbors = 0
    for i_aux in range(i - 1, i + 2, 1):
        i_aux = i_aux % len(state)
        for j_aux in range(j - 1, j + 2, 1):
            j_aux = j_aux % len(state[i])
            for k_aux in range(k - 1, k + 2, 1):
                k_aux = k_aux % len(state[i][j])
                if not (i_aux == i and j_aux == j and k_aux == k) \
                        and state[i_aux][j_aux][k_aux]:
                    alive_neighbors += 1
    return alive_neighbors


def process_state(state: list) -> list:
    new_state = []
    for i in range(len(state)):
        new_state.append([])
        for j in range(len(state[i])):
            new_state[i].append([])
            for k in range(len(state[i][j])):
                alive_neighbors = get_alive_neighbors(state, i, j, k)
                if state[i][j][k] and not (2 <= alive_neighbors <= 3):
                    new_state[i][j].append(False)
                elif not state[i][j][k] and alive_neighbors == 3:
                    new_state[i][j].append(True)
                else:
                    new_state[i][j].append(state[i][j][k])
    return new_state


def advance_state_dimensions(state: list) -> list:
    return [[[False for _j in range(len(state[0][0]) + 2)] for _i in range(len(state[0]) + 2)]] \
           + [[[False for _j in range(len(state[0][0]) + 2)]] + [[False] + state_line + [False] for state_line in
                                                                 state_aux] + [
                  [False for _j in range(len(state[0][0]) + 2)]] for state_aux in state] \
           + [[[False for _j in range(len(state[0][0]) + 2)] for _i in range(len(state[0]) + 2)]]


def printable_state(state: list) -> str:
    board = ''
    for i in range(len(state)):
        board += f'z={i - len(state) // 2}\n'
        for j in range(len(state[i])):
            for k in range(len(state[i][j])):
                if state[i][j][k]:
                    board += '#'
                else:
                    board += '.'
            board += '\n'
    return board


def get_active_cells(state):
    alive_cells = 0
    for i in range(len(state)):
        for j in range(len(state[i])):
            for k in range(len(state[i][j])):
                if state[i][j][k]:
                    alive_cells += 1
    return alive_cells


state = [[]]
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            state[0].append(parse_line(line.strip()))
print(state)
print(f'Initial State:\n {printable_state(state)}')
for i in range(6):
    print(f'Turn {i + 1}:')
    state = advance_state_dimensions(state)
    new_state = process_state(state)
    state = new_state
    print(f'`{printable_state(state)}')
    print()
print(f'{get_active_cells(state)}')
