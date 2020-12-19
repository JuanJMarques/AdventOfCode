numbers_spoken = [6, 19, 0, 5, 7, 13, 1]
mem = dict()
last_number = numbers_spoken[len(numbers_spoken) - 1]
turn = len(numbers_spoken)
for i in range(len(numbers_spoken)):
    if numbers_spoken[i] not in mem:
        mem[numbers_spoken[i]] = []
    mem[numbers_spoken[i]].append(i)
    if len(mem[numbers_spoken[i]]) > 2:
        mem[numbers_spoken[i]] = mem[numbers_spoken[i]][1:]
for _i in range(len(numbers_spoken), 30000000, 1):
    new_number = 0
    if len(mem[last_number]) > 1:
        new_number = mem[last_number][1] - mem[last_number][0]
    if new_number not in mem:
        mem[new_number] = []
    mem[new_number].append(turn)
    if len(mem[new_number]) > 2:
        mem[new_number] = mem[new_number][1:]
    turn += 1
    last_number = new_number
print('the 30000000th number spoken is {}'.format(last_number))

#     if numbers_spoken.count(last_number) == 1:
#         numbers_spoken.append(0)
#     else:
#         reverse = numbers_spoken[:-1][::-1]
#         numbers_spoken.append(reverse.index(last_number) + 1)
# print('the 2020th number spoken is {}'.format(numbers_spoken[-1]))
# print(numbers_spoken)
# asd = set(numbers_spoken)
# dsa = dict()
# for a in asd:
#     dsa[a] = numbers_spoken.count(a)
# print(dsa)
print("done")
