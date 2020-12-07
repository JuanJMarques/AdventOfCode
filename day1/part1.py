lines = open('input.txt')

numbers = []
test = 2020
for line in lines:
    numbers.append(int(line))

for i in range(len(numbers)-1):
    for j in range(i+1, len(numbers)):
        if numbers[i]+numbers[j] == test:
            print('found! numbers {} and {} sums {}'.format(numbers[i], numbers[j],test))
            print('{} * {} = {}'.format(numbers[i], numbers[j],numbers[i]*numbers[j]))
