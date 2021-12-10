import re as regex


def simplify_rule(rules: dict, rule: str, simplified_rules: dict = dict()):
    if rule.startswith('"'):
        return rule.replace('"', '').replace('"', '')
    simplified_rule = ''
    rule_parts = rule.split(' ')
    put_parenthesis = True
    for part in rule_parts:
        if part == '|':
            simplified_rule += '|'
            # put_parenthesis = True
        elif part.isdigit():
            if part not in simplified_rules:
                simplified_part = simplify_rule(rules, rules[part], simplified_rules)
                simplified_rules[part] = simplified_part
            simplified_rule += simplified_rules[part]
    if put_parenthesis:
        simplified_rule = f'({simplified_rule})'
    return simplified_rule


def process_rules(rules: dict) -> str:
    main_rule = rules['0']
    simplified_rules = dict()
    simplified_rules['0'] = simplify_rule(rules, main_rule, simplified_rules)
    return simplified_rules['0']


rules = dict()
parse_rules = True
messages = []
with open('input.txt') as lines:
    for line in lines:
        if line.strip() != '':
            if parse_rules:
                parts = line.strip().split(':')
                name = parts[0].strip()
                rule = parts[1].strip()
                rules[name] = rule
            else:
                messages.append(line.strip())
        elif parse_rules:
            parse_rules = False
main_rule = process_rules(rules)
print(f'Main rule: {main_rule}')
matched_messages = 0
for message in messages:
    if regex.match(main_rule, message):
        matched_messages += 1
        print(f'message: "{message}" match main rule')
print(f'number of messages that match main rule: {matched_messages}')
