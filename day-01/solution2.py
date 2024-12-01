file_path = input("Input file path: ")

xs, ys = [], []

with open(file_path, "r") as f:
    lines = f.readlines()
    for line in lines:
        x, y = line.split()
        xs.append(int(x))
        ys.append(int(y))

S = 0
for x in xs:
    occurences = ys.count(x)
    S += x * occurences
print(S)
