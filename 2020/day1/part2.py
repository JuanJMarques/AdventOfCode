lines = open('input.txt')

numbers = []
test = 2020
for line in lines:
    numbers.append(int(line))

for i in range(len(numbers)-2):
    for j in range(i+1, len(numbers)-1):
        for k in range(j+1, len(numbers)):
            if numbers[i]+numbers[j]+numbers[k] == test:
                print('found! numbers {}, {} & {} sums {}'.format(numbers[i], numbers[j], numbers[k],test))
                print('{} * {} * {}= {}'.format(numbers[i], numbers[j],numbers[k],numbers[i]*numbers[j]*numbers[k]))