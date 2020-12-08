total_affirmative = 0
with open('input.txt') as lines:
    affirmative_answers = set()
    for line in lines:
        if line.strip() == '':
            total_affirmative += len(affirmative_answers)
            affirmative_answers = set()
        else:
            answers = list(line.strip())
            for answer in answers:
                affirmative_answers.add(answer)
print('total affirmative answers: {}'.format(total_affirmative))
