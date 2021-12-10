def connect_adapters(adapters):
    curr_joltage = 0
    diff_1_amount = 0
    diff_3_amount = 0
    for adapter in adapters:
        diff = adapter - curr_joltage
        if diff > 3:
            print("Error: difference between curr joltage {} & adpater {} is {}".format(curr_joltage, adapter, diff))
            exit(-1)
        if diff == 3:
            diff_3_amount += 1
        elif diff == 1:
            diff_1_amount += 1
        curr_joltage = adapter
    return diff_1_amount, diff_3_amount + 1


adapters = []
with open('input.txt') as lines:
    for line in lines:
        if '' != line.strip():
            in_number = int(line.strip())
            adapters.append(in_number)
diff_1_amount, diff_3_amount = connect_adapters(sorted(adapters))
print('number of 1 jolt diffs: {}'.format(diff_1_amount))
print('number of 3 jolts diffs: {}'.format(diff_3_amount))
print('multiplied differences: {}'.format(diff_1_amount * diff_3_amount))
