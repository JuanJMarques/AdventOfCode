import re

valid_passports = 0


def check_byr(byr):
    return 1920 <= byr <= 2002


def check_iyr(iyr):
    return 2010 <= iyr <= 2020


def check_eyr(eyr):
    return 2020 <= eyr <= 2030


def check_hgt(hgt):
    hgt = str(hgt)
    if not (hgt.endswith('cm') or hgt.endswith('in')):
        return False
    if hgt.endswith('cm'):
        height = int(hgt.replace('cm', ''))
        return 150 <= height <= 193
    else:
        height = int(hgt.replace('in', ''))
        return 59 <= height <= 76


def check_hcl(hcl):
    return re.match('^#[0-9a-f]{6}', hcl) is not None


def check_ecl(ecl):
    return ecl == 'amb' \
           or ecl == 'blu' \
           or ecl == 'brn' \
           or ecl == 'gry' \
           or ecl == 'grn' \
           or ecl == 'hzl' \
           or ecl == 'oth'


def check_pid(pid):
    return re.match('[0-9]{9}', pid) is not None


def check_fields(passport_fields):
    # byr (Birth Year)
    # iyr (Issue Year)
    # eyr (Expiration Year)
    # hgt (Height)
    # hcl (Hair Color)
    # ecl (Eye Color)
    # pid (Passport ID)
    # cid (Country ID)
    if 'byr' in passport_fields.keys() \
            and 'iyr' in passport_fields.keys() \
            and 'eyr' in passport_fields.keys() \
            and 'hgt' in passport_fields.keys() \
            and 'hcl' in passport_fields.keys() \
            and 'ecl' in passport_fields.keys() \
            and 'pid' in passport_fields.keys():
        return check_byr(int(passport_fields['byr'])) \
               and check_iyr(int(passport_fields['iyr'])) \
               and check_eyr(int(passport_fields['eyr'])) \
               and check_hgt(passport_fields['hgt']) \
               and check_hcl(passport_fields['hcl']) \
               and check_ecl(passport_fields['ecl']) \
               and check_pid(passport_fields['pid'])
    return False


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
