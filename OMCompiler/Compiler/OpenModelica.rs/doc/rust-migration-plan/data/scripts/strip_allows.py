import sys, re
root, lst = sys.argv[1], sys.argv[2]
n = 0
for rel in open(lst):
    p = root + '/' + rel.strip()
    s = open(p).read()
    t = s.replace('#![allow(warnings)]\n', '', 1)
    t = re.sub(r'#!\[allow\(\s*unreachable_patterns,.*?\)\]\n', '', t, count=1, flags=re.S)
    if t != s: n += 1; open(p, 'w').write(t)
print('stripped', n)
