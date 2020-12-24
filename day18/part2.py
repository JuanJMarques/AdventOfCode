def search_closing_parenthesis(operations: str, begin: int) -> int:
    level = 0
    index = begin
    while index < len(operations):
        if operations[index] == '(':
            level += 1
        elif operations[index] == ')':
            level -= 1
            if level <= 0:
                return index
        index += 1
    pass


def process_operations(operations: str, index: int = 0) -> (int, int):
    count = 0
    op = '+'
    while index < len(operations):
        if operations[index].isdigit():
            if op == '+':
                count += int(operations[index])
            elif op == '*':
                count *= int(operations[index])
        elif operations[index] == '(':
            end = search_closing_parenthesis(operations, index)
            par_count, _ = process_operations(operations[index + 1:end])
            index = end
            if op == '+':
                count += par_count
            elif op == '*':
                count *= par_count
        elif operations[index] == ')':
            return count, index
        elif operations[index] == '+':
            op = operations[index]
        elif operations[index] == '*':
            par_count, index = process_operations(operations, index + 1)
            count *= par_count
        index += 1
    return count, index


total = 0
result = 0
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            result, _ = process_operations(line.strip().replace(' ', ''))
            print(f'{line.strip()} = {result}')
            total += result
print(f'total results added = {total}')
