c = int(input())

score = {}

for i in range(c):
    for i in input().split()[1:]:
        if i not in score:
            score[i] = 0
        score[i] += 1
    for i in input().split()[1:]:
        if i not in score:
            score[i] = 0
        score[i] -= 1

pizza = [i for i, s in score.items() if s >= 0]

print(len(pizza), *pizza)
