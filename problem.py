zero = [0xF0,
0x90,
0x90,
0x90,
0xF0]
one = [0x20,
    0x60,
    0x20,
    0x20,
    0x70]

d = [0xE0,
0x90,
0x90,
0x90,
0xE0]
present = [[0 for i in range(8)] for j in range(5)]

for i in range(5):
    for j in range(8):
        present[i][8-1-j] ^= (d[i] >> j) & 1

for line in present:
    print(line)