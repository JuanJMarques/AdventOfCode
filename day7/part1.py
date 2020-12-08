import re as regex


def can_contain_shiny_gold(bags, containing_bags):
    if 'shiny gold' in containing_bags:
        return True
    for bag in containing_bags:
        if can_contain_shiny_gold(bags, bags[bag]):
            return True
    return False


bags = dict()
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            line = line.replace('\n', '').replace('.', '')
            parts = line.split('contain')
            bag = parts[0].replace(' bags', '').strip()
            if 'no other' == parts[1].replace(' bags', '').replace(' bag', '').strip():
                bags[bag] = []
            else:
                contains = [s.strip() for s in regex.sub('\d+', '', parts[1].replace(' bags', '').replace(' bag', '')).split(',')]
                bags[bag] = contains

bags_can_contain_shiny_gold = 0
for bag in bags.keys():
    if can_contain_shiny_gold(bags, bags[bag]):
        bags_can_contain_shiny_gold += 1
print('number of bags which can contain shiny gold is: {}'.format(bags_can_contain_shiny_gold))
