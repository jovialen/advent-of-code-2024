file_path = input("Input file path: ")

xs, ys = [], []

with open(file_path, "r") as f:
    lines = f.readlines()
    for line in lines:
        x, y = line.split()
        xs.append(int(x))
        ys.append(int(y))

xs.sort()
ys.sort()

pairs = zip(xs, ys)
distances = list(map(lambda a: abs(a[0] - a[1]), pairs))

print(sum(distances))
