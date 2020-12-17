mem = dict()
mask = []
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            if line.startswith('mask'):
                mask = list(line.split('=')[1].strip())
            else:
                addresses = [int(line.split('=')[0].strip().replace('mem[', '').replace(']', ''))]
                value = int(line.split('=')[1].strip())
                for i in range(len(mask)):
                    j = len(mask) - i - 1
                    if mask[j] == '1':
                        addresses = [address | 2 ** i for address in addresses]
                    if mask[j] == 'X':
                        addresses = [address | 2 ** i for address in addresses] \
                                    + [address & (-1 ^ 2 ** i) for address in addresses]
                for address in addresses:
                    mem[address] = value
sum = 0
for address in mem:
    sum += mem[address]
print("The sum of the values in memory is {}".format(sum))
