start_tickets = False
start_nearby = False
rules = dict()
rule_index = 0
nearby_passports = []
with open('input.txt') as lines:
    for line in lines:
        if not start_tickets:
            if line.strip() == '':
                start_tickets = True
            else:
                ranges = line.split(':')[1].strip().split('or')
                rules[rule_index] = []
                for i in range(len(ranges)):
                    rules[rule_index].append(range(int(ranges[i].split('-')[0]), int(ranges[i].split('-')[1]) + 1, 1))
                rule_index += 1
        elif not start_nearby:
            if line.startswith('nearby tickets'):
                start_nearby = True
        else:
            nearby_passports.append([int(field) for field in line.strip().split(',')])
scanning_error_rate = 0
for passport in nearby_passports:
    for passport_field in passport:
        valid_field = False
        for rule_index in rules:
            for rule in rules[rule_index]:
                valid_field |= passport_field in rule
        if not valid_field:
            scanning_error_rate += passport_field
print('the scanning error rate is {}'.format(scanning_error_rate))
