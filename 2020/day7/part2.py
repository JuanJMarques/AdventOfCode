import re as regex


def can_contain_shiny_gold(bags, containing_bags):
    if 'shiny gold' in containing_bags:
        return True
    for bag in containing_bags:
        if can_contain_shiny_gold(bags, bags[bag]):
            return True
    return False


def count_bags(bag_name, bags, bags_number):
    if len(bags[bag_name]) == 0:
        return 0
    count = 0
    for bag_index in range(len(bags[bag_name])):
        curr_bag = bags[bag_name][bag_index]
        count += bags_number[bag_name][bag_index] + bags_number[bag_name][bag_index] * count_bags(curr_bag,bags,bags_number)
    return count


bags = dict()
bags_number = dict()
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            line = line.replace('\n', '').replace('.', '')
            parts = line.split('contain')
            bag = parts[0].replace(' bags', '').strip()
            if 'no other' == parts[1].replace(' bags', '').replace(' bag', '').strip():
                bags[bag] = []
            else:
                contains = [s.strip() for s in regex.sub('\d+', '', parts[1].replace(' bags', '').replace(' bag', ''))
                    .split(',')]
                bags[bag] = contains
                numbers = [int(regex.findall('\d+',s)[0]) for s in parts[1].replace(' bags', '').replace(' bag', '').split(',')]
                bags_number[bag] = numbers
print('number of bags in shiny gold bags {}'.format(count_bags('shiny gold', bags, bags_number)))
