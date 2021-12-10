valid_pass = 0
with open('input.txt') as lines:
    for line in lines:
        # here the policy means the number of times the letter must appear in the password
        policy = line.split(':')[0]
        password = line.split(':')[1].strip()
        min_times = int(policy.split(' ')[0].split('-')[0])
        max_times = int(policy.split(' ')[0].split('-')[1])
        letter = policy.split(' ')[1]
        matches = password.count(letter)
        if min_times <= matches <= max_times:
            print("password '{}' match policy '{}'".format(password, policy))
            valid_pass += 1
print('number of passwords that match policy: {}'.format(valid_pass))
