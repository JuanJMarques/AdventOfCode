def search_occupied_visible(seats: list, i: int, j: int, i_direction, j_direction) -> int:
    i += i_direction
    j += j_direction
    if i < 0 or j < 0 or i >= len(seats) or j >= len(seats[0]):
        return 0
    if seats[i][j] == 'L':
        return 0
    elif seats[i][j] == '#':
        return 1
    if seats[i][j] == '.':
        return search_occupied_visible(seats, i, j, i_direction, j_direction)


def get_occupied_visible(seats: list, i: int, j: int) -> int:
    occupied_visible = 0
    occupied_visible += search_occupied_visible(seats, i, j, -1, -1)
    occupied_visible += search_occupied_visible(seats, i, j, -1, 0)
    occupied_visible += search_occupied_visible(seats, i, j, -1, 1)
    occupied_visible += search_occupied_visible(seats, i, j, 0, -1)
    occupied_visible += search_occupied_visible(seats, i, j, 0, 1)
    occupied_visible += search_occupied_visible(seats, i, j, 1, -1)
    occupied_visible += search_occupied_visible(seats, i, j, 1, 0)
    occupied_visible += search_occupied_visible(seats, i, j, 1, 1)
    return occupied_visible


def step(seats: list) -> list:
    new_seats = []
    for i in range(len(seats)):
        seats_line = seats[i]
        new_seats_line = []
        for j in range(len(seats_line)):
            occupied_visible = get_occupied_visible(seats, i, j)
            if 'L' == seats_line[j] and occupied_visible == 0:
                new_seats_line.append('#')
            elif '#' == seats_line[j] and occupied_visible >= 5:
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
