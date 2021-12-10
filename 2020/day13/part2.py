def get_inverse_mod(factor: int, mod: int) -> tuple:
    s, t, sp, tp = (1, 0, 0, 1)
    fact1 = factor
    fact2 = mod
    while fact2 != 0:
        div, res = divmod(fact1, fact2)
        fact1 = fact2
        fact2 = res
        st = s
        s = sp
        sp = st - sp * div
        tt = t
        t = tp
        tp = tt - tp * div
    return fact1, s, t


def search_earliest_timestamp(bus_lines: list) -> int:
    N = 1
    for line in bus_lines:
        N *= line
    X = [N // line for line in bus_lines]
    X_inv = []
    time = 0
    for i in range(len(bus_lines)):
        X_inv.append(get_inverse_mod(X[i] % bus_lines[i], bus_lines[i])[1] % bus_lines[i])
        time += ((bus_lines[i] - i) * X[i] * X_inv[i])
    return time % N


_earliest_depart = None
bus_lines = []
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            if _earliest_depart is None:
                _earliest_depart = int(line.strip())
            else:
                for line_name in line.strip().split(','):
                    if line_name != 'x':
                        bus_lines.append(int(line_name))
                    else:
                        bus_lines.append(1)
time = search_earliest_timestamp(bus_lines)
print('Sol: {}'.format(time))
