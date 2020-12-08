total_affirmative = 0
with open('input.txt') as lines:
    affirmative_answers = set()
    start = True
    for line in lines:
        if line.strip() == '':
            total_affirmative += len(affirmative_answers)
            affirmative_answers = set()
            start = True
        else:
            line_affirmative_answers = set()
            answers = list(line.strip())
            for answer in answers:
                line_affirmative_answers.add(answer)
            # line_affirmative_answers = set(list(line.strip()))
            if start:
                affirmative_answers = set(line_affirmative_answers)
                start = False
            affirmative_answers = affirmative_answers.intersection(line_affirmative_answers)
print('total affirmative answers: {}'.format(total_affirmative))
