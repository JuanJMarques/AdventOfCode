trees_number = 0
index_x = 0
tree_char = '#'
with open('input.txt') as lines:
    for line in lines:
        line = line.strip()
        l = list(line)
        if l[index_x] == tree_char:
            trees_number += 1
            l[index_x] = 'X'
        else:
            l[index_x] = '0'
        print("".join(l))
        index_x += 3
        index_x = index_x % len(line)
print('number of trees encountered: {}'.format(trees_number))