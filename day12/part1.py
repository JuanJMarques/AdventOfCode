def process_instruction(inst: str, amount: int, north: int, south: int, east: int, west: int, facing: str):
    directions = ['E', 'S', 'W', 'N']
    if 'N' == inst:
        return north + amount, south, east, west, facing
    if 'S' == inst:
        return north, south + amount, east, west, facing
    if 'E' == inst:
        return north, south, east + amount, west, facing
    if 'W' == inst:
        return north, south, east, west + amount, facing
    if 'F' == inst:
        return process_instruction(facing, amount, north, south, east, west, facing)
    if 'R' == inst:
        return north, south, east, west, directions[(directions.index(facing) + amount // 90) % len(directions)]
    if 'L' == inst:
        return north, south, east, west, directions[(directions.index(facing) - amount // 90) % len(directions)]


north = 0
south = 0
east = 0
west = 0
facing = 'E'
with open('input.txt') as lines:
    for line in lines:
        if not line.strip() == '':
            inst = line[0]
            amount = int(line[1:])
            north, south, east, west, facing = process_instruction(inst, amount, north, south, east, west, facing)
print('Manhattan distance: {}'.format(abs(north - south) + abs(east - west)))
