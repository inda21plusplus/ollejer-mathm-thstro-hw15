pizza = "4 cheese mushrooms tomatoes peppers"

points = 0
ingredients = pizza.split()[1:]
customers = int(input())

for i in range(customers):
    likes = input().split()[1:]
    dislikes = input().split()[1:]
    for topping in likes:
        print(topping, likes, not topping in likes)
        if not topping in ingredients:
            print("missing good ingredient")
    for topping in dislikes:
        print(topping, likes, topping in likes)
        if topping in ingredients:
            continue
    points += 1

print(points)