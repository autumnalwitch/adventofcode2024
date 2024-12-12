file = open("input.txt")

def isSafe(list):
    is_safe = True
    is_increasing = list[0] < list[1]
    # print(is_increasing)
    for i in range(len(list) - 1):
        diff = list[i+1] - list[i]
        if is_increasing:
            if not (1 <= abs(diff) <= 3 and diff > 0):# these ones are not increasing:
                is_safe = False
        else:
            if not (1 <= abs(diff) <= 3 and diff < 0):# these ones are not decreaing:
                is_safe = False
    return is_safe

total = 0
for line in file:
    line = line.rstrip()

    # split the line into two ints
    list = line.split()
    list = [int(x) for x in list]
    # print(list)
    is_safe = isSafe(list)
    if not is_safe:
        for i in range(len(list)):
            inner_list = list.copy()
            del inner_list[i]
            is_safe = isSafe(inner_list)
            if is_safe:
                break
    total += is_safe 

print(total)

    