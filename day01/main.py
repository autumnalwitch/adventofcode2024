file = open("input.txt")
list_a = []
list_b = []
list_s = []
for line in file:
    line = line.rstrip()
    # split the line into two ints
    list = line.split()
    list_a.append(int(list[0]))
    list_b.append(int(list[1]))

list_a.sort()
list_b.sort()

solution = 0
for i in range(len(list_a)):
    solution = solution + abs(list_b[i] - list_a[i])

print(solution)
