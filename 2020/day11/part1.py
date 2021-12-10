def get_occupied_adjacents(seats: list, i: int, j: int) -> int:
    occupied_adjacents = 0
    if i > 0:
        if j > 0:
            if seats[i - 1][j - 1] == '#':
                occupied_adjacents += 1
        if seats[i - 1][j] == '#':
            occupied_adjacents += 1
        if j + 1 < len(seats[i]):
            if seats[i - 1][j + 1] == '#':
                occupied_adjacents += 1
    if j > 0:
        if seats[i][j - 1] == '#':
            occupied_adjacents += 1
    if j + 1 < len(seats[i]):
        if seats[i][j + 1] == '#':
            occupied_adjacents += 1
    if i + 1 < len(seats):
        if j > 0:
            if seats[i + 1][j - 1] == '#':
                occupied_adjacents += 1
        if seats[i + 1][j] == '#':
            occupied_adjacents += 1
        if j + 1 < len(seats[i]):
            if seats[i + 1][j + 1] == '#':
                occupied_adjacents += 1
    return occupied_adjacents


def step(seats: list) -> list:
    new_seats = []
    for i in range(len(seats)):
        seats_line = seats[i]
        new_seats_line = []
        for j in range(len(seats_line)):
            occupied_adjacents = get_occupied_adjacents(seats, i, j)
            if 'L' == seats_line[j] and occupied_adjacents == 0:
                new_seats_line.append('#')
            elif '#' == seats_line[j] and occupied_adjacents >= 4:
                new_seats_line.append('L')
            else:
                new_seats_line.append(seats_line[j])
        new_seats.append(new_seats_line)
    return new_seats


def count_occupied_seats(seats: list) -> int:
    occupied_seats = 0
    for seat_line in seats:
        for seat in seat_line:
            if '#' == seat:
                occupied_seats += 1
    return occupied_seats


seats = []
with open('input.txt') as lines:
    for line in lines:
        if not line.strip() == '':
            seats.append(list(line.strip()))
new_seats = []
started = False
while not started or not seats == new_seats:
    if started:
        seats = new_seats
    else:
        started = True
    new_seats = step(seats)
print("the number of seats occupied is {}".format(count_occupied_seats(seats)))
