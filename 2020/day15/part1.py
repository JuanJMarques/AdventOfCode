numbers_spoken = [6, 19, 0, 5, 7, 13, 1]
for _i in range(len(numbers_spoken), 2020, 1):
    last_number = numbers_spoken[len(numbers_spoken) - 1]
    if numbers_spoken.count(last_number) == 1:
        numbers_spoken.append(0)
    else:
        reverse = numbers_spoken[:-1][::-1]
        numbers_spoken.append(reverse.index(last_number) + 1)
print('the 2020th number spoken is {}'.format(numbers_spoken[-1]))
