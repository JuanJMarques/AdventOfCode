invalid_sum = 22406676

buffer = []


def search_for_range_that_sums_invalid(numbers, sum_to_search):
    sub_buff_size = 2
    found = False
    while not found:
        for i in range(len(buffer) + 1 - sub_buff_size):
            sum_buff = sum(numbers[i:i + sub_buff_size])
            if sum_buff == sum_to_search:
                return numbers[i: i + sub_buff_size]
        sub_buff_size += 1


with open('input.txt') as lines:
    for line in lines:
        in_number = int(line.strip())
        if invalid_sum == in_number:
            sub_buff = search_for_range_that_sums_invalid(buffer, invalid_sum)
            print("Sum of {} is equal to {}".format(sub_buff, invalid_sum))
            print("Max element: {}. Min element: {}".format(max(sub_buff), min(sub_buff)))
            print("Sum of max & min = {}".format(max(sub_buff) + min(sub_buff)))
            exit(0)
        else:
            buffer.append(in_number)
