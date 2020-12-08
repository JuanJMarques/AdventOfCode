def check_trees(horizontal_step=3, vertical_step=1):
    tree_char = '#'
    index_x = 0
    trees_number = 0
    curr_vertical_step = vertical_step
    begin = True
    with open('input.txt') as lines:
        for line in lines:
            if not begin and curr_vertical_step > 1:
                curr_vertical_step -= 1
            else:
                begin = False
                curr_vertical_step = vertical_step
                line = line.strip()
                l = list(line)
                if l[index_x] == tree_char:
                    trees_number += 1
                    l[index_x] = 'X'
                else:
                    l[index_x] = '0'
                # print("".join(l))
                index_x += horizontal_step
                index_x = index_x % len(line)
    return trees_number


trees_1_1 = check_trees(1, 1)
trees_3_1 = check_trees(3, 1)
trees_5_1 = check_trees(5, 1)
trees_7_1 = check_trees(7, 1)
trees_1_2 = check_trees(1, 2)
print('number of trees encountered with step 1-1: {}'.format(trees_1_1))
print('number of trees encountered with step 3-1: {}'.format(trees_3_1))
print('number of trees encountered with step 5-1: {}'.format(trees_5_1))
print('number of trees encountered with step 7-1: {}'.format(trees_7_1))
print('number of trees encountered with step 1-2: {}'.format(trees_1_2))
print('multiplied result: {}'.format(trees_1_1*trees_3_1*trees_5_1*trees_7_1*trees_1_2))

