#!/usr/bin/env python3
"""Translate SimulationRuntime/c/RuntimeSources.mo.cmake into RuntimeSources.rs.cmake.

usage: runtime_sources_rs.py <RuntimeSources.mo.cmake> <RuntimeSources.rs.cmake>
"""
import re
import sys

HEADER = '''// Configured by SimulationRuntime/c/CMakeLists.txt into RuntimeSources.rs.

/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(non_upper_case_globals)]

use arcstr::ArcStr;
use metamodelica::List;
use std::sync::LazyLock;

fn strs(items: &[&'static str]) -> List<ArcStr> {
    items.iter().map(|s| ArcStr::from(*s)).collect()
}
'''

ITEM = re.compile(
    r'(//[^\n]*)|constant\s+(String|list<String>)\s+(\w+)\s*=\s*(\{.*?\}|"[^"]*")\s*;',
    re.S)


def translate(src):
    body = src.split('encapsulated package RuntimeSources', 1)[1]
    body = body.rsplit('annotation(', 1)[0]
    out = []
    for m in ITEM.finditer(body):
        comment, ty, name, value = m.groups()
        if comment:
            if out and not out[-1].startswith('//'):
                out.append('')
            out.append(comment)
            continue
        if out and not out[-1].startswith('//'):
            out.append('')
        if ty == 'String':
            out.append(f'pub const {name}: &str = {value};')
            continue
        inner = value[1:-1].strip()
        if re.fullmatch(r'@\w+@', inner):
            out.append(f'pub static {name}: LazyLock<List<ArcStr>> =\n'
                       f'    LazyLock::new(|| strs(&[{inner}]));')
        else:
            items = ''.join(f'        {s},\n' for s in re.findall(r'"[^"]*"', inner))
            out.append(f'pub static {name}: LazyLock<List<ArcStr>> = LazyLock::new(|| {{\n'
                       f'    strs(&[\n{items}    ])\n}});')
    # Anything left over is a declaration the patterns above do not know.
    rest = ITEM.sub('', body).strip()
    if rest:
        sys.exit(f'runtime_sources_rs.py: cannot translate: {rest[:200]}')
    return HEADER + '\n' + '\n'.join(out) + '\n'


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    with open(sys.argv[1], encoding='utf-8') as f:
        text = translate(f.read())
    with open(sys.argv[2], 'w', encoding='utf-8') as f:
        f.write(text)


if __name__ == '__main__':
    main()
