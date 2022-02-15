import sys

pizza = input().split()[1:]
with open(sys.argv[1]) as f:
    lines = f.read().split("\n")

points = 0
for likes, dislikes in zip(lines[1::2], lines[2::2]):
    pizza_ok = True
    for topping in likes.split()[1:]:
        if not topping in pizza:
            pizza_ok = False
    for topping in dislikes.split()[1:]:
        if topping in pizza:
            pizza_ok = False
    if pizza_ok:
        points += 1


print(points)
