def move_waypoint(way_point: list, direction: int):
    new_way_point = [s for s in way_point]
    for i in range(len(way_point)):
        new_way_point[(i + direction) % len(way_point)] = way_point[i]
    return new_way_point


def process_instruction(inst: str, amount: int, north: int, south: int, east: int, west: int, way_point: list):
    if 'N' == inst:
        way_point[3] += amount
        return north, south, east, west, way_point
    if 'S' == inst:
        way_point[1] += amount
        return north, south, east, west, way_point
    if 'E' == inst:
        way_point[0] += amount
        return north, south, east, west, way_point
    if 'W' == inst:
        way_point[2] += amount
        return north, south, east, west, way_point
    if 'F' == inst:
        return north + way_point[3] * amount, south + way_point[1] * amount, east + way_point[0] * amount, west + \
               way_point[2] * amount, way_point
    if 'R' == inst:
        direction = (amount // 90) % len(way_point)
        way_point = move_waypoint(way_point, direction)
        return north, south, east, west, way_point
    if 'L' == inst:
        direction = (amount // 90) % len(way_point)
        way_point = move_waypoint(way_point, -direction)
        return north, south, east, west, way_point


north = 0
south = 0
east = 0
west = 0
facing = 'E'
# ['E', 'S', 'W', 'N']
waypoint = [10, 0, 0, 1]
with open('input.txt') as lines:
    for line in lines:
        if not line.strip() == '':
            inst = line[0]
            amount = int(line[1:])
            north, south, east, west, waypoint = process_instruction(inst, amount, north, south, east, west, waypoint)
print('Manhattan distance: {}'.format(abs(north - south) + abs(east - west)))
