#!/usr/bin/env python3

# Based on harfbuzz/src/gen-arabic-table.py

import os
import urllib.request

DEPENDENCIES = [
    "ArabicShaping.txt",
    "UnicodeData.txt",
    "Blocks.txt",
]

for dep in DEPENDENCIES:
    if not os.path.exists(dep):
        urllib.request.urlretrieve("https://unicode.org/Public/14.0.0/ucd/" + dep, dep)

files = [open(x, encoding="utf-8") for x in DEPENDENCIES]

headers = [
    [files[0].readline(), files[0].readline()],
    [files[2].readline(), files[2].readline()],
    ["UnicodeData.txt does not have a header."],
]
while files[0].readline().find("##################") < 0:
    pass

blocks = {}


def read_blocks(f):
    global blocks
    for line in f:
        j = line.find("#")
        if j >= 0:
            line = line[:j]

        fields = [x.strip() for x in line.split(";")]
        if len(fields) == 1:
            continue

        uu = fields[0].split("..")
        start = int(uu[0], 16)
        if len(uu) == 1:
            end = start
        else:
            end = int(uu[1], 16)

        t = fields[1]

        for u in range(start, end + 1):
            blocks[u] = t


def print_joining_table(f):
    values = {}
    for line in f:
        if line[0] == "#":
            continue

        fields = [x.strip() for x in line.split(";")]
        if len(fields) == 1:
            continue

        u = int(fields[0], 16)

        if fields[3] in ["ALAPH", "DALATH RISH"]:
            value = "JOINING_GROUP_" + fields[3].replace(" ", "_")
        else:
            value = "JOINING_TYPE_" + fields[2]
        values[u] = value

    short_value = {}
    for value in sorted(set([v for v in values.values()] + ["JOINING_TYPE_X"])):
        short = "".join(x[0] for x in value.split("_")[2:])
        assert short not in short_value.values()

        short_value[value] = short

    uu = sorted(values.keys())
    num = len(values)
    all_blocks = set([blocks[u] for u in uu])

    last = -100000
    ranges = []
    for u in uu:
        if u - last <= 1 + 16 * 5:
            ranges[-1][-1] = u
        else:
            ranges.append([u, u])
        last = u

    print("#[rustfmt::skip]")
    print("pub const JOINING_TABLE: &[JoiningType] = &[")
    last_block = None
    offset = 0

    join_offsets = []

    for start, end in ranges:
        join_offsets.append(
            "const JOINING_OFFSET_0X%04X: usize = %d;" % (start, offset)
        )

        for u in range(start, end + 1):
            block = blocks.get(u, last_block)
            value = values.get(u, "JOINING_TYPE_X")

            if block != last_block or u == start:
                if u != start:
                    print()
                if block in all_blocks:
                    print("\n  /* %s */" % block)
                else:
                    print("\n  /* FILLER */")
                last_block = block
                if u % 32 != 0:
                    print()
                    print("  /* %04X */" % (u // 32 * 32), "  " * (u % 32), end="")

            if u % 32 == 0:
                print()
                print("  /* %04X */ " % u, end="")

            val = short_value[value]

            if val == "C":
                val = "D"

            print("%s," % val, end="")
        print()

        offset += end - start + 1
    print("];")
    print()

    for offset in join_offsets:
        print(offset)

    page_bits = 12
    print()
    print("pub fn joining_type(u: char) -> JoiningType {")
    print("    let u = u as u32;")
    print("    match u >> %d {" % page_bits)
    pages = set(
        [u >> page_bits for u in [s for s, e in ranges] + [e for s, e in ranges]]
    )
    for p in sorted(pages):
        print("        0x%0X => {" % p)
        for start, end in ranges:
            if p not in [start >> page_bits, end >> page_bits]:
                continue
            offset = "JOINING_OFFSET_0X%04X" % start
            print("            if (0x%04X..=0x%04X).contains(&u) {" % (start, end))
            print(
                "                return JOINING_TABLE[u as usize - 0x%04X + %s]"
                % (start, offset)
            )
            print("            }")
        print("        }")
    print("        _ => {}")
    print("    }")
    print()
    print("    X")
    print("}")
    print()


print("// WARNING: this file was generated by ../scripts/gen-arabic-table.py")
print()
print(
    "use super::arabic::JoiningType::{self, GroupAlaph as A, GroupDalathRish as DR, D, L, R, T, U, X};"
)
print()

read_blocks(files[2])
print_joining_table(files[0])
