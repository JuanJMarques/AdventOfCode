instruction_mem = []


def load_program(instruction_mem):
    with open('input2.txt') as lines:
        for line in lines:
            instruction_mem.append(line.strip())


load_program(instruction_mem)


def exec_instruction(instruction, pc, acc):
    print('exectuting instuction {}:{}'.format(pc, instruction))
    [code, val] = instruction.split(' ')
    if 'nop' == code:
        return pc + 1, acc
    if 'acc' == code:
        return pc + 1, acc + int(val)
    if 'jmp' == code:
        return pc + int(val), acc
    raise Exception('Instruction: {} not known'.format(instruction))


def exec_code(instruction_mem, save_history=False):
    print('=======================BEGINING EXECUTION=======================')
    visited_instructions = []
    pc = 0
    acc = 0
    while 0 <= pc < len(instruction_mem):
        if pc in visited_instructions:
            print('Loop detected at instruction {}. Current acc value: {}'.format(pc, acc))
            print('max instruction visited {}'.format(max(visited_instructions)))
            if save_history:
                return pc, acc, False, visited_instructions
            return pc, acc, False
        visited_instructions.append(pc)
        pc, acc = exec_instruction(instruction_mem[pc], pc, acc)
    if save_history:
        return pc, acc, True, visited_instructions
    return pc, acc, True


def get_instruction_to_change(instruction_mem, pc):
    while True:
        instruction = instruction_mem[pc]
        [code, val] = instruction.split(' ')
        if 'nop' == code:
            return pc, '{} {}'.format('jmp', val)
        if 'jmp' == code:
            return pc, '{} {}'.format('nop', val)
        pc += 1


pc, acc, finished, pc_history = exec_code(instruction_mem, True)
print('current registers values - PC: [{}] ACC: [{}]'.format(pc, acc))
changed_instruction = -1
new_instruction = None
while not finished:
    changed_instruction, new_instruction = get_instruction_to_change(instruction_mem, changed_instruction + 1)
    old_instruction = instruction_mem[changed_instruction]
    instruction_mem[changed_instruction] = new_instruction
    pc, acc, finished = exec_code(instruction_mem)
    instruction_mem[changed_instruction] = old_instruction
    print('current registers values - PC: [{}] ACC: [{}]'.format(pc, acc))
print('Program Successfully executed.')
print('Changed instruction {} from {} to {}'.format(changed_instruction, instruction_mem[changed_instruction],
                                                    new_instruction))
