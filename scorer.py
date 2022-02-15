import sys

pizza = open(sys.argv[1])
pizza = pizza.read()
ingredients = pizza.split()[1:]
points = 0
customers = int(input())

for _ in range(customers):
    pizza_ok = True
    likes = input().split()[1:]
    dislikes = input().split()[1:]
    for topping in likes:
        if not topping in ingredients:
            pizza_ok = False
    for topping in dislikes:
        if topping in ingredients:
            pizza_ok = False
    if pizza_ok:
        points += 1


print(points)
