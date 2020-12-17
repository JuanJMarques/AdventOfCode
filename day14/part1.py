mem = [0 for i in range(999999)]
mask = []
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            if line.startswith('mask'):
                mask = list(line.split('=')[1].strip())
            else:
                address = int(line.split('=')[0].strip().replace('mem[', '').replace(']', ''))
                value = int(line.split('=')[1].strip())
                for i in range(len(mask)):
                    j = len(mask) - i - 1
                    if mask[j] == '1':
                        value = value | 2 ** i
                    if mask[j] == '0':
                        value = value & (-1 ^ 2 ** i)
                mem[address] = value
sum = 0
for val in mem:
    sum += val
print("The sum of the values in memory is {}".format(sum))
