def check_sum(number_check, number_buffer, min_index, max_index):
    if min_index == max_index:
        return False
    min = number_buffer[min_index]
    max = number_buffer[max_index]
    sum = min + max
    if number_check == sum:
        return True
    if number_check > sum:
        return check_sum(number_check, number_buffer, min_index + 1, max_index)
    if number_check < sum:
        return check_sum(number_check, number_buffer, min_index, max_index - 1)


def check_number_is_sum_of_2_buffer_numbers(number, numbers):
    numbers = sorted(numbers)
    return check_sum(number, numbers, 0, len(numbers) - 1)


buffer = []
with open('input.txt') as lines:
    for line in lines:
        in_number = int(line.strip())
        if len(buffer) >= 25:
            if not check_number_is_sum_of_2_buffer_numbers(in_number, buffer):
                print("Current number {} isn't sum of 2 numbers in the buffer".format(in_number))
                exit(0)
            buffer = buffer[1:]
        buffer.append(in_number)
