import re


def search_seat(directions, min, max, down_direction_letter):
    if min == max or len(directions) == 0:
        return min
    direction = directions[0]
    half = (min + max + 1) // 2
    if direction == down_direction_letter:
        return search_seat(directions[1:], min, half - 1, down_direction_letter)
    else:
        return search_seat(directions[1:], half, max, down_direction_letter)


def get_row(directions):
    return search_seat(directions, 0, 127, 'F')


def get_column(directions):
    return search_seat(directions, 0, 7, 'L')


def get_seat_id(line):
    row = get_row(line[:7])
    column = get_column(line[7:])
    return row * 8 + column
    pass


seats = [False for i in range(128*8)]
with open('input.txt') as lines:
    passport_fields = dict()
    for line in lines:
        line = line.strip()
        if line != '' and re.match('[F|B]{7}[L|R]{3}', line) is not None:
            seat_id = get_seat_id(list(line))
            seats[seat_id] = True

front_empty = True
back_emty = False
for i in range(len(seats)):
    if not seats[i]:
        if not (front_empty or back_emty):
            print('Seat {} is empty'.format(i))
            back_emty = True
    else:
        front_empty = False
