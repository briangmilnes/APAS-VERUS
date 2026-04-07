# Veracity Style Rule [18] Fix: Understand Multi-Type Section Cycling

## Context

You are working in `~/projects/veracity`. Do not touch `~/projects/veracity-agent1`.

## Problem

Rule [18] fires 718 times. Almost all are false positives. The rule compares
section numbers globally — "section 4 should come before section 9" — but the
APAS-VERUS standard uses **per-type-group cycling**: sections 4-10 repeat for
each type, with letter suffixes (a, b, c...).

In a multi-type file, this is the correct order:

```
Section 4a. type definitions — struct Leaf
Section 5a. view impls — struct Leaf
Section 6a. spec fns — struct Leaf
Section 7a. proof fns — struct Leaf
Section 8a. traits — struct Leaf
Section 9a. impls — struct Leaf
Section 4b. type definitions — struct Tree     ← 4b AFTER 9a is CORRECT
Section 5b. view impls — struct Tree
...
Section 9b. impls — struct Tree
Section 10b. iterators — struct Tree
Section 11. top level coarse locking
Section 12a. derive impls in verus! — struct Leaf
Section 12b. derive impls in verus! — struct Tree
```

The styler currently sees `Section 4b` after `Section 9a` and reports:
"struct Tree should come before impls (expected type definitions before impls)"
This is WRONG. Section 4b correctly follows section 9a because it's a new
type group.

## The Standard

Read `tests/fixtures/APAS-VERUS/src/standards/table_of_contents_standard.rs`.
Get a fresh fixture first:

```bash
rm -rf tests/fixtures/APAS-VERUS
git clone https://github.com/briangmilnes/APAS-VERUS.git tests/fixtures/APAS-VERUS
```

The ordering rule from the standard:

1. Sections 1-3 are global (one per file): module, imports, broadcast use.
2. Sections 4-10 repeat per type group (a, b, c...), ordered leaf-first
   (bottom-up). Each type gets a complete 4-10 cycle.
3. Section 11 appears once (Mt modules only), after all type groups.
4. Sections 12-14 repeat per type group, bottom-up. These come after
   section 11 (or after the last section 10 if no section 11).

The valid ordering is:

```
1 < 2 < 3 < 4a < 5a < 6a < 7a < 8a < 9a < 10a
                < 4b < 5b < 6b < 7b < 8b < 9b < 10b
                < 4c < ...
                < 11
                < 12a < 12b < 12c
                < 13a < 13b < 13c
                < 14a < 14b < 14c
```

Within a type group, sections are ordered: 4 < 5 < 6 < 7 < 8 < 9 < 10.
Across type groups, the ENTIRE cycle of group N comes before group N+1.
Sections 1-3 come before all type groups. Sections 11-14 come after all
type groups.

## What to fix

### Step 1: Parse section headers with letter suffixes

The rule must parse section headers like `Section 7b. proof fns` and extract
both the number (7) and the suffix (b). Currently it only looks at the number.

### Step 2: Define the ordering relation

Two items are correctly ordered if:

```
fn is_correctly_ordered(prev: SectionId, curr: SectionId) -> bool {
    match (prev, curr) {
        // Global sections: 1 < 2 < 3, before any type group
        (Global(n1), Global(n2)) => n1 < n2,
        (Global(_), TypeGroup(_, _)) => true,
        (Global(_), Post(_, _)) => true,

        // Within same type group: section number must increase
        (TypeGroup(g1, s1), TypeGroup(g2, s2)) if g1 == g2 => s1 < s2,

        // Across type groups: any section in group N+1 follows any in group N
        (TypeGroup(g1, _), TypeGroup(g2, _)) if g1 < g2 => true,

        // Type group before post sections (11-14)
        (TypeGroup(_, _), Post(_, _)) => true,

        // Post sections: 11 < 12a < 12b < 13a < 13b < 14a < 14b
        (Post(s1, g1), Post(s2, g2)) => s1 < s2 || (s1 == s2 && g1 < g2),

        _ => false,
    }
}
```

Where:
- `Global(n)` = sections 1, 2, 3 (no suffix)
- `TypeGroup(group_letter, section_number)` = sections 4-10 with suffix
- `Post(section_number, group_letter)` = sections 11-14

### Step 3: Single-type files

Files with only one type group use bare section numbers (no letter suffix):
`Section 4. type definitions` instead of `Section 4a. type definitions`.
These are equivalent to a single type group "a". The ordering is just
1 < 2 < 3 < 4 < 5 < 6 < 7 < 8 < 9 < 10 < 11 < 12 < 13 < 14.

### Step 4: Omitted sections

Not all sections appear in every file. Missing sections are fine — the rule
checks that PRESENT sections are in the correct relative order, not that all
sections exist.

## Expected impact

~650 of 718 [18] warnings should become info or disappear. The remaining
~68 would be genuine ordering violations (e.g., a spec fn appearing after
an impl within the SAME type group).

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse section headers
with proper awareness. A string-hacking detector will flag and kill tools that
corrupt source syntax.
