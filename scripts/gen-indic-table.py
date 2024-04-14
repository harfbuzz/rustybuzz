#!/usr/bin/env python3

# Based on harfbuzz/src/gen-indic-table.py

import io
import os
import urllib.request

DEPENDENCIES = [
    'IndicSyllabicCategory.txt',
    'IndicPositionalCategory.txt',
    'Blocks.txt',
]

for dep in DEPENDENCIES:
    if not os.path.exists(dep):
        urllib.request.urlretrieve('https://unicode.org/Public/14.0.0/ucd/' + dep, dep)

ALLOWED_SINGLES = [0x00A0, 0x25CC]
ALLOWED_BLOCKS = [
    'Basic Latin',
    'Latin-1 Supplement',
    'Devanagari',
    'Bengali',
    'Gurmukhi',
    'Gujarati',
    'Oriya',
    'Tamil',
    'Telugu',
    'Kannada',
    'Malayalam',
    'Sinhala',
    'Myanmar',
    'Khmer',
    'Vedic Extensions',
    'General Punctuation',
    'Superscripts and Subscripts',
    'Devanagari Extended',
    'Myanmar Extended-B',
    'Myanmar Extended-A',
]

files = [io.open(x, encoding='utf-8') for x in DEPENDENCIES]

headers = [[f.readline() for i in range(2)] for f in files]

data = [{} for f in files]
values = [{} for f in files]
for i, f in enumerate(files):
    for line in f:
        j = line.find('#')
        if j >= 0:
            line = line[:j]

        fields = [x.strip() for x in line.split(';')]
        if len(fields) == 1:
            continue

        uu = fields[0].split('..')
        start = int(uu[0], 16)
        if len(uu) == 1:
            end = start
        else:
            end = int(uu[1], 16)

        t = fields[1]

        for u in range(start, end + 1):
            data[i][u] = t
        values[i][t] = values[i].get(t, 0) + end - start + 1

# Merge data into one dict:
defaults = ('Other', 'Not_Applicable', 'No_Block')
for i, v in enumerate(defaults):
    values[i][v] = values[i].get(v, 0) + 1

combined = {}
for i, d in enumerate(data):
    for u, v in d.items():
        if i == 2 and u not in combined:
            continue
        if u not in combined:
            combined[u] = list(defaults)
        combined[u][i] = v
combined = {k: v for k, v in combined.items() if k in ALLOWED_SINGLES or v[2] in ALLOWED_BLOCKS}
data = combined
del combined
num = len(data)

# Move the outliers NO-BREAK SPACE and DOTTED CIRCLE out
singles = {}
for u in ALLOWED_SINGLES:
    singles[u] = data[u]
    del data[u]

print('// WARNING: this file was generated by scripts/gen-indic-table.py')
print()
print('#![allow(non_camel_case_types)]')
print('#![allow(unused_imports)]')
print()
print('use super::ot_shaper_indic::{MatraCategory, SyllabicCategory};')

# Shorten values
short = [{
    'Bindu': 'Bi',
    'Cantillation_Mark': 'Ca',
    'Joiner': 'ZWJ',
    'Non_Joiner': 'ZWNJ',
    'Number': 'Nd',
    'Visarga': 'Vs',
    'Vowel': 'Vo',
    'Vowel_Dependent': 'M',
    'Consonant_Prefixed': 'CPrf',
    'Other': 'x',
}, {
    'Not_Applicable': 'x',
}]
all_shorts = [{}, {}]

# Add some of the values, to make them more readable, and to avoid duplicates

for i in range(2):
    for v, s in short[i].items():
        all_shorts[i][s] = v

what = ['SyllabicCategory', 'MatraCategory']
what_short = ['ISC', 'IMC']
cat_defs = []
for i in range(2):
    vv = sorted(values[i].keys())
    for v in vv:
        v_no_and = v.replace('_And_', '_')
        if v in short[i]:
            s = short[i][v]
        else:
            s = ''.join([c for c in v_no_and if ord('A') <= ord(c) <= ord('Z')])
            if s in all_shorts[i]:
                raise Exception('Duplicate short value alias', v, all_shorts[i][s])
            all_shorts[i][s] = v
            short[i][v] = s
        cat_defs.append((what_short[i] + '_' + s, what[i] + '::' + v.replace('_', ''), str(values[i][v]), v))

maxlen_s = max([len(c[0]) for c in cat_defs])
maxlen_l = max([len(c[1]) for c in cat_defs])
maxlen_n = max([len(c[2]) for c in cat_defs])
for s in what_short:
    print()
    for c in [c for c in cat_defs if s in c[0]]:
        print('use %s as %s;' % (c[1].ljust(maxlen_l), c[0]))
print()
print()

total = 0
used = 0
last_block = None


def print_block(block, start, end, data):
    global total, used, last_block
    if block and block != last_block:
        print()
        print()
        print('  /* %s */' % block)
    num = 0
    assert start % 8 == 0
    assert (end + 1) % 8 == 0
    for u in range(start, end + 1):
        if u % 8 == 0:
            print()
            print('  /* %04X */' % u, end='')
        if u in data:
            num += 1
        d = data.get(u, defaults)
        print('%16s' % ('(ISC_%s,IMC_%s),' % (short[0][d[0]], short[1][d[1]])), end='')

    total += end - start + 1
    used += num
    if block:
        last_block = block


uu = sorted(data.keys())

last = -100000
num = 0
offset = 0
starts = []
ends = []
print('#[rustfmt::skip]')
print('const TABLE: &[(SyllabicCategory, MatraCategory)] = &[')
offsets = []
for u in uu:
    if u <= last:
        continue
    block = data[u][2]

    start = u // 8 * 8
    end = start + 1
    while end in uu and block == data[end][2]:
        end += 1
    end = (end - 1) // 8 * 8 + 7

    if start != last + 1:
        if start - last <= 1 + 16 * 3:
            print_block(None, last + 1, start - 1, data)
            last = start - 1
        else:
            if last >= 0:
                ends.append(last + 1)
                offset += ends[-1] - starts[-1]
            # print()
            # print()
            offsets.append('const OFFSET_0X%04X: usize = %d;' % (start, offset))
            starts.append(start)

    print_block(block, start, end, data)
    last = end
ends.append(last + 1)
offset += ends[-1] - starts[-1]
print()
print()
occupancy = used * 100. / total
page_bits = 12
print('];')
print()
for o in offsets:
    print(o)
print()
print('#[rustfmt::skip]')
print('pub fn get_categories(u: u32) -> (SyllabicCategory, MatraCategory) {')
print('    match u >> %d {' % page_bits)
pages = set([u >> page_bits for u in starts + ends + list(singles.keys())])
for p in sorted(pages):
    print('        0x%0X => {' % p)
    for u, d in singles.items():
        if p != u >> page_bits: continue
        print('            if u == 0x%04X { return (ISC_%s, IMC_%s); }' % (u, short[0][d[0]], short[1][d[1]]))
    for (start, end) in zip(starts, ends):
        if p not in [start >> page_bits, end >> page_bits]: continue
        offset = 'OFFSET_0X%04X' % start
        print('            if (0x%04X..=0x%04X).contains(&u) { return TABLE[u as usize - 0x%04X + %s]; }' % (start, end - 1, start, offset))
    print('        }')
print('        _ => {}')
print('    }')
print()
print('    (ISC_x, IMC_x)')
print('}')

# Maintain at least 30% occupancy in the table */
if occupancy < 30:
    raise Exception('Table too sparse, please investigate: ', occupancy)
