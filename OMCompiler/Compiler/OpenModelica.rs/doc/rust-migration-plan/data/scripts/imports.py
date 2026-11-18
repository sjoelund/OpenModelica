import re, os, sys, json, collections
root = os.environ["REPO"] + "/OMCompiler/Compiler"
srcs = [l.strip() for l in open(os.environ["BUILD"] + "/OMCompiler/Compiler/rust_compilerSources.txt") if l.strip() and not l.startswith('#')]
mods = {}
imp_re = re.compile(r'^\s*(?:public\s+|protected\s+)?import\s+(?:\w+\s*=\s*)?([A-Za-z_][\w]*)', re.M)
iface_re = re.compile(r'__OpenModelica_Interface\s*=\s*"(\w+)"')
for p in srcs:
    name = os.path.basename(p)[:-3]
    rel = os.path.relpath(p, root)
    d = rel.split('/')[0] if '/' in rel else '.'
    if 'generated-mo' in p: d = p.split('generated-mo/')[1].split('/')[0]
    elif 'build-cmake' in p: d = 'Util'
    elif '/../SimulationRuntime' in p: d = 'SimulationRuntime'
    txt = open(p, errors='replace').read()
    # strip comments
    t = re.sub(r'/\*.*?\*/', '', txt, flags=re.S)
    t = re.sub(r'//[^\n]*', '', t)
    imps = set(imp_re.findall(t)) - {name}
    m = iface_re.search(txt)
    mods[name] = dict(dir=d, lines=txt.count('\n'), imports=sorted(imps), iface=m.group(1) if m else None)
json.dump(mods, open(os.path.join(os.path.dirname(__file__), 'mods.json'), 'w'), indent=1)
print(len(mods))
c = collections.Counter(m['dir'] for m in mods.values())
print(c)
