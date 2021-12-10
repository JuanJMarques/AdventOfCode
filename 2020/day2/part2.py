valid_pass = 0
with open('input.txt') as lines:
    for line in lines:
        # here the policy means the positions where the letter could be but only in 1 of the 2 index.
        # the index count starts at 1
        policy = line.split(':')[0]
        password = line.split(':')[1].strip()
        first_index = int(policy.split(' ')[0].split('-')[0])
        second_index = int(policy.split(' ')[0].split('-')[1])
        letter = policy.split(' ')[1]
        if (password[first_index - 1] == letter and password[second_index - 1] != letter) \
                or (password[first_index - 1] != letter and password[second_index - 1] == letter):
            print("password '{}' match policy '{}'".format(password, policy))
            valid_pass += 1
print('number of passwords that match policy: {}'.format(valid_pass))
