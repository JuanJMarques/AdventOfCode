def process_operations(operations: str, index: int) -> (int, int):
    count = 0
    op = '+'
    while index < len(operations):
        if operations[index].isdigit():
            if op == '+':
                count += int(operations[index])
            elif op == '*':
                count *= int(operations[index])
        elif operations[index] == '(':
            par_count, index = process_operations(operations, index + 1)
            if op == '+':
                count += par_count
            elif op == '*':
                count *= par_count
        elif operations[index] == ')':
            return count, index
        elif operations[index] == '+' or operations[index] == '*':
            op = operations[index]
        index += 1
    return count, index


total = 0
result = 0
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            result, _ = process_operations(line.strip().replace(' ', ''), 0)
            print(f'{line.strip()} = {result}')
            total += result
print(f'total results added = {total}')
