print(sum([abs(a - b) for a,b in list(zip(*[sorted(list(a)) for a in zip(*[(int(f[0]), int(f[1])) for f in [f.split("   ") for f in open("inputs/1_input.txt", "r")]])]))]))

print((lambda c: sum([a * c[1].count(a) for a in c[0]]))([sorted(list(a)) for a in zip(*[(int(f[0]), int(f[1])) for f in [f.split("   ") for f in open("inputs/1_input.txt", "r")]])]))
