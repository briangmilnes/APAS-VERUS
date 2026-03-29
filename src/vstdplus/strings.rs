//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Verified wrappers for Rust's char and String methods.
//! Each function wraps the real Rust method with an external_body bridge and spec.
//! ASCII-only for now — sufficient for DocumentIndex and textbook examples.

pub mod strings {

    use vstd::prelude::*;

    verus! {

    // =========================================================================
    // char specs — wrapping Rust's char methods
    // =========================================================================

    /// Spec for `char::is_ascii_alphabetic()`.
    /// Matches Rust: U+0041 'A' ..= U+005A 'Z', U+0061 'a' ..= U+007A 'z'.
    pub open spec fn spec_char_is_ascii_alphabetic(c: char) -> bool {
        (c as u32) >= 0x41 && (c as u32) <= 0x5A      // A-Z
        || (c as u32) >= 0x61 && (c as u32) <= 0x7A    // a-z
    }

    /// Spec for `char::to_ascii_lowercase()`.
    /// Matches Rust: uppercase ASCII → lowercase, else self.
    pub open spec fn spec_char_to_ascii_lowercase(c: char) -> char {
        if (c as u32) >= 0x41 && (c as u32) <= 0x5A {
            ((c as u32) + 32) as char
        } else {
            c
        }
    }

    /// Spec for `char::is_ascii_whitespace()`.
    /// Matches Rust: U+0009 (tab), U+000A (LF), U+000C (FF), U+000D (CR), U+0020 (space).
    pub open spec fn spec_char_is_ascii_whitespace(c: char) -> bool {
        (c as u32) == 0x09 || (c as u32) == 0x0A || (c as u32) == 0x0C
        || (c as u32) == 0x0D || (c as u32) == 0x20
    }

    /// Spec for `char::is_ascii()`.
    /// Matches Rust: U+0000 ..= U+007F.
    pub open spec fn spec_char_is_ascii(c: char) -> bool {
        (c as u32) <= 0x7F
    }

    // char exec bridges — each wraps the real Rust method

    /// Wraps `char::is_ascii_alphabetic()`.
    #[verifier::external_body]
    pub fn char_is_ascii_alphabetic(c: char) -> (b: bool)
        ensures b == spec_char_is_ascii_alphabetic(c)
    { c.is_ascii_alphabetic() }

    /// Wraps `char::to_ascii_lowercase()`.
    #[verifier::external_body]
    pub fn char_to_ascii_lowercase(c: char) -> (lc: char)
        ensures lc == spec_char_to_ascii_lowercase(c)
    { c.to_ascii_lowercase() }

    /// Wraps `char::is_ascii_whitespace()`.
    #[verifier::external_body]
    pub fn char_is_ascii_whitespace(c: char) -> (b: bool)
        ensures b == spec_char_is_ascii_whitespace(c)
    { c.is_ascii_whitespace() }

    /// Wraps `char::is_ascii()`.
    #[verifier::external_body]
    pub fn char_is_ascii(c: char) -> (b: bool)
        ensures b == spec_char_is_ascii(c)
    { c.is_ascii() }

    // =========================================================================
    // String/str specs — wrapping Rust's String and str methods
    // =========================================================================

    /// Spec for lowercasing a sequence of chars.
    pub open spec fn spec_to_ascii_lowercase_seq(s: Seq<char>) -> Seq<char> {
        s.map(|_i: int, c: char| spec_char_to_ascii_lowercase(c))
    }

    // String exec bridges

    /// Wraps `str::chars()` — returns chars as Vec for indexed iteration with specs.
    #[verifier::external_body]
    pub fn str_to_chars(s: &str) -> (chars: Vec<char>)
        ensures chars@.len() >= 0
    { s.chars().collect() }

    /// Wraps `String::as_str()` → chars. Convenience for String input.
    #[verifier::external_body]
    pub fn string_to_chars(s: &String) -> (chars: Vec<char>)
        ensures chars@.len() >= 0
    { s.chars().collect() }

    /// Wraps `str::to_ascii_lowercase()` — returns lowercased chars.
    #[verifier::external_body]
    pub fn str_to_ascii_lowercase_chars(s: &str) -> (chars: Vec<char>)
        ensures
            chars@.len() >= 0,
            forall|i: int| 0 <= i < chars@.len() ==>
                #[trigger] spec_char_is_ascii(chars@[i]),
    { s.to_ascii_lowercase().chars().collect() }

    /// Wraps `String::new()`.
    pub fn string_new() -> (s: String)
    { String::new() }

    /// Wraps `String::push(char)` — tracked via ghost Seq<char> on caller side.
    #[verifier::external_body]
    pub fn string_push(s: &mut String, c: char)
    { s.push(c); }

    /// Wraps `String::clear()`.
    #[verifier::external_body]
    pub fn string_clear(s: &mut String)
    { s.clear(); }

    /// Wraps `String::is_empty()`.
    #[verifier::external_body]
    pub fn string_is_empty(s: &String) -> (b: bool)
    { s.is_empty() }

    /// Wraps `String::len()` — byte length, not char count.
    #[verifier::external_body]
    pub fn string_len(s: &String) -> (n: usize)
    { s.len() }

    // =========================================================================
    // Vec<char> buffer utilities — for building strings char-by-char with specs
    // =========================================================================

    /// Create empty char buffer.
    pub fn new_char_buf() -> (buf: Vec<char>)
        ensures buf@.len() == 0
    { Vec::new() }

    /// Check if char buffer is empty.
    pub fn char_buf_is_empty(buf: &Vec<char>) -> (b: bool)
        ensures b == (buf@.len() == 0)
    { buf.len() == 0 }

    /// Push char onto buffer — Vec<char> has full Verus specs via vstd.
    pub fn char_buf_push(buf: &mut Vec<char>, c: char)
        ensures buf@ == old(buf)@.push(c)
    { buf.push(c); }

    /// Clear char buffer.
    #[verifier::external_body]
    pub fn char_buf_clear(buf: &mut Vec<char>)
        ensures buf@.len() == 0
    { buf.clear(); }

    /// Convert char buffer to String.
    #[verifier::external_body]
    pub fn char_buf_to_string(buf: &Vec<char>) -> (s: String)
    { buf.iter().collect() }

    } // verus!
} // mod
