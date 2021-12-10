def remove_invalid_passports(nearby_passports: list, rules: dict) -> list:
    removed_passports = []
    for passport in nearby_passports:
        for passport_field in passport:
            valid_field = False
            for rule_index in rules:
                for rule in rules[rule_index]:
                    valid_field |= passport_field in rule
            if not valid_field:
                removed_passports.append(passport)
    for passport in removed_passports:
        nearby_passports.remove(passport)
    return nearby_passports


def filter_columns(candidate_columns: list) -> list:
    sorted_columns = sorted(candidate_columns, key=lambda columns: len(columns))
    for i in range(len(sorted_columns)):
        if len(sorted_columns[i]) > 1:
            print("Error")
        else:
            for j in range(len(candidate_columns)):
                if j != candidate_columns.index(sorted_columns[i]) and sorted_columns[i][0] in candidate_columns[j]:
                    candidate_columns[j].remove(sorted_columns[i][0])
                if i != j and sorted_columns[i][0] in sorted_columns[j]:
                    sorted_columns[j].remove(sorted_columns[i][0])
    return [k[0] for k in candidate_columns]


def identify_passport_columns(passports: list, rules: dict, rule_indexes: list) -> list:
    columns = []
    candidate_columns = [[] for _i in rules]
    for i in range(len(rules)):
        rule = rules[i]
        for j in range(len(passports[0])):

            column_valid = True
            if j not in columns:
                for passport in passports:
                    is_in_range = False
                    for rule_range in rule:
                        is_in_range |= passport[j] in rule_range
                    column_valid &= is_in_range
                if column_valid:
                    candidate_columns[i].append(j)
    discovered_columns = filter_columns(candidate_columns)
    for index in rule_indexes:
        columns.append(discovered_columns[index])
    return columns


start_tickets = False
start_nearby = False
start_own = False
rules = dict()
rule_index = 0
own_passport = []
nearby_passports = []
departure_rules = []
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
                if line.startswith('departure'):
                    departure_rules.append(rule_index)
                rule_index += 1
        elif not start_nearby:
            if line.startswith('nearby tickets'):
                start_nearby = True
            elif not start_own:
                if line.startswith('your ticket:'):
                    start_own = True
            else:
                if len(own_passport) == 0 and line.strip() != '':
                    own_passport = [int(field) for field in line.strip().split(',')]
        else:
            nearby_passports.append([int(field) for field in line.strip().split(',')])
valid_nearby_passports = remove_invalid_passports(nearby_passports, rules)
departure_columns = identify_passport_columns(nearby_passports, rules, departure_rules)

mult = 1
for column in departure_columns:
    mult = mult * own_passport[column]
print(f'the six departure columns multiplied are {mult}')
