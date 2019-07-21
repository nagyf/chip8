import sys

with open(sys.argv[1], 'rb') as f:
    contents = f.read()

result = []
for byte in contents:
    result.append('0x%.2x' % byte)

print(', '.join(result))
