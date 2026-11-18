import sys, json, collections
gen = set(l.strip() for l in open(sys.argv[1]))
out = sys.argv[2]
by_lint = collections.Counter(); by_lint_gen = collections.Counter(); by_lint_mach = collections.Counter()
by_crate = collections.Counter(); by_crate_lint = collections.defaultdict(collections.Counter)
levels = collections.Counter(); errors = []
n = 0
def mach(msg):
    for c in msg.get('children', []):
        for sp in c.get('spans', []):
            if sp.get('suggestion_applicability') == 'MachineApplicable': return True
    for sp in msg.get('spans', []):
        if sp.get('suggestion_applicability') == 'MachineApplicable': return True
    return False
for line in sys.stdin:
    if not line.startswith('{'): continue
    try: j = json.loads(line)
    except Exception: continue
    if j.get('reason') != 'compiler-message': continue
    m = j['message']; lvl = m.get('level')
    levels[lvl] += 1
    if lvl == 'error':
        errors.append(m.get('rendered', '')[:400]); continue
    if lvl != 'warning': continue
    code = (m.get('code') or {}).get('code') or ('<summary>' if m['message'].startswith(('`', 'unused', '')) and not m.get('spans') else m['message'][:60])
    if not m.get('spans'): continue
    crate = j['target']['name']
    f = m['spans'][0]['file_name']
    rel = f.split('OpenModelica.rs/')[-1]
    isgen = rel in gen
    n += 1
    by_lint[code] += 1; by_crate[crate] += 1; by_crate_lint[crate][code] += 1
    if isgen: by_lint_gen[code] += 1
    if mach(m): by_lint_mach[code] += 1
json.dump(dict(total=n, levels=levels, by_lint=by_lint, by_lint_gen=by_lint_gen, by_lint_mach=by_lint_mach,
               by_crate=by_crate, by_crate_lint={k: dict(v) for k, v in by_crate_lint.items()}, errors=errors[:50]),
          open(out, 'w'), indent=1)
print('warnings', n, 'levels', dict(levels))
