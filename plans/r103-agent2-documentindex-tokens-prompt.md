# R103 Agent 2 (continued) — Prove DocumentIndex tokens + make_index, STEP 20

## Objective

DocumentIndex has 1 remaining hole: `make_index` (external_body). The function
calls `tokens()` (also external_body, outside verus!) then sorts and builds a table.

Prove `tokens` by moving it inside verus! using the new vstd string specs +
vstdplus/strings.rs wraps. Then prove or reduce `make_index`.

## What vstd provides (String View = Seq<char>)

- `String@` returns `Seq<char>` — full View support
- `str::chars()` → `Chars` iterator with View specs
- `Chars::next()` → `Option<char>` with `ensures` (index advances, returns next char)
- `String::new()` → ensures `s@ == Seq::empty()`
- `String::append(&mut self, other: &str)` → ensures `self@ == old(self)@ + other@`
- `String::from_str(s)` → ensures `ret@ == s@`
- `String::clone()` → ensures `ret@ == s@`
- `String::eq()` → ensures `ret == (s@ == other@)`

## What vstdplus/strings.rs provides (just merged)

- `char_is_ascii_alphabetic(c)` → ensures `b == spec_char_is_ascii_alphabetic(c)`
- `char_to_ascii_lowercase(c)` → ensures `lc == spec_char_to_ascii_lowercase(c)`
- `string_push(s, c)` → ensures `s@ == old(s)@.push(c)`
- `string_clear(s)` → ensures `s@ == Seq::<char>::empty()`
- `string_is_empty(s)` → ensures `b == (s@.len() == 0)`

## Proving tokens

Current `tokens` (outside verus!):
```rust
pub fn tokens(content: &Contents) -> ArraySeqStPerS<Word> {
    let content_lower = content.to_lowercase();
    for ch in content_lower.chars() {
        if ch.is_alphabetic() { current_word.push(ch); }
        else if !current_word.is_empty() { emit word; reset; }
    }
}
```

Rewrite inside verus!:
1. Use `str_to_ascii_lowercase_chars` or iterate with vstd's `Chars` and
   `char_to_ascii_lowercase` per char
2. Use `char_is_ascii_alphabetic` for the test
3. Use `string_push` to build current word
4. Use `string_is_empty` to check for emit
5. Collect words into Vec, then `ArraySeqStPerS::from_vec`

The loop invariant tracks: words collected so far, current_word state.
Since tokens has `ensures true` (no spec on output), the invariant just
needs to maintain wf.

## Proving make_index

`make_index` calls `tokens`, then `sort_unstable_by`, then builds the table.
`sort_unstable_by` has NO Verus spec. Options:
- Keep `make_index` as external_body but with tokens verified inside it
- Write a verified sort (we have one in Chap36) — overkill
- Use external_body on just the sort step

Realistic target: move `tokens` inside verus!, prove it. Keep `make_index`
external_body but note that only `sort_unstable_by` blocks it.

## Read first

- `src/Chap44/DocumentIndex.rs` — tokens (line 196), make_index (line 98)
- `src/vstdplus/strings.rs` — the 5 wraps
- `~/projects/verus/source/vstd/string.rs` — vstd String/str/Chars specs

## Isolation

```bash
scripts/validate.sh isolate Chap44
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add assume or accept.
- Proving tokens inside verus! is the main win even if make_index stays
  external_body.
- Use vstd's Chars iterator if possible. If too complex, use
  `str_to_chars` to get Vec<char> and iterate with a while loop.

## STEP 20

## Report

Write `plans/agent2-r103-tokens-report.md`.
