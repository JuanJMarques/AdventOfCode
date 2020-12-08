valid_passports = 0


def check_fields(passport_fields):
    # byr (Birth Year)
    # iyr (Issue Year)
    # eyr (Expiration Year)
    # hgt (Height)
    # hcl (Hair Color)
    # ecl (Eye Color)
    # pid (Passport ID)
    # cid (Country ID)
    return 'byr' in passport_fields.keys() \
           and 'iyr' in passport_fields.keys() \
           and 'eyr' in passport_fields.keys() \
           and 'hgt' in passport_fields.keys() \
           and 'hcl' in passport_fields.keys() \
           and 'ecl' in passport_fields.keys() \
           and 'pid' in passport_fields.keys()


def populate(passport_fields, fields):
    for field in fields:
        key = field.split(':')[0]
        val = field.split(':')[1]
        passport_fields[key] = val


with open('input.txt') as lines:
    passport_fields = dict()
    for line in lines:
        if line.strip() == '':
            if check_fields(passport_fields):
                valid_passports += 1
            passport_fields = dict()
        else:
            fields = line.replace('\n', '').split(' ')
            populate(passport_fields, fields)
print('number of valid passports: {}'.format(valid_passports))
