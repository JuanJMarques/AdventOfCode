instruction_mem = []


def load_program(instruction_mem):
    with open('input.txt') as lines:
        for line in lines:
            instruction_mem.append(line.strip())


load_program(instruction_mem)


def exec_instruction(instruction, pc, acc):
    [code, val] = instruction.split(' ')
    if 'nop' == code:
        return pc + 1, acc
    if 'acc' == code:
        return pc + 1, acc + int(val)
    if 'jmp' == code:
        return pc + int(val), acc
    raise Exception('Instruction: {} not known'.format(instruction))


def exec_code(instruction_mem):
    visited_instructions = []
    pc = 0
    acc = 0
    while 0 <= pc < len(instruction_mem):
        if pc in visited_instructions:
            print('Loop detected at instruction {}. Current acc value: {}'.format(pc, acc))
            return pc, acc
        visited_instructions.append(pc)
        pc, acc = exec_instruction(instruction_mem[pc], pc, acc)
    return pc, acc


pc, acc = exec_code(instruction_mem)
print('current registers values - PC: [{}] ACC: [{}]'.format(pc, acc))
