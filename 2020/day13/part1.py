earliest_depart = None
bus_lines = []
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            if earliest_depart is None:
                earliest_depart = int(line.strip())
            else:
                for line_name in line.strip().split(','):
                    if line_name != 'x':
                        bus_lines.append(int(line_name))

lines_mods = [earliest_depart % bus_line for bus_line in bus_lines]
if 0 in lines_mods:
    bus_index = lines_mods.index(0)
else:
    bus_index = lines_mods.index(max(lines_mods))
print('the earliest bus you can take is {} with {} minutes waiting'.format(bus_lines[bus_index],
                                                                           bus_lines[bus_index] - lines_mods[
                                                                               bus_index]))
print('multiplied values are {}'.format(bus_lines[bus_index] * (bus_lines[bus_index] - lines_mods[bus_index])))
