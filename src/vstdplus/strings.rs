//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Verified wraps for Rust char and String methods not yet in vstd.
//! Each function wraps exactly one Rust method with an external_body bridge.
//! vstd already provides: String/str View = Seq<char>, chars() iterator,
//! String::new/clone/eq/from_str/append/concat, str::len/is_ascii.

pub mod strings {

    use vstd::prelude::*;

    verus! {

    // Spec fns for char methods.

    /// Spec for `char::is_ascii_alphabetic()`.
    /// Matches Rust: U+0041 'A' ..= U+005A 'Z', U+0061 'a' ..= U+007A 'z'.
    pub open spec fn spec_char_is_ascii_alphabetic(c: char) -> bool {
        (c as u32) >= 0x41 && (c as u32) <= 0x5A
        || (c as u32) >= 0x61 && (c as u32) <= 0x7A
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

    // Exec wraps for char methods.

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

    // Exec wraps for String methods not in vstd.

    /// Wraps `String::push(char)`.
    #[verifier::external_body]
    pub fn string_push(s: &mut String, c: char)
        ensures s@ == old(s)@.push(c)
    { s.push(c); }

    /// Wraps `String::clear()`.
    #[verifier::external_body]
    pub fn string_clear(s: &mut String)
        ensures s@ == Seq::<char>::empty()
    { s.clear(); }

    /// Wraps `String::is_empty()`.
    #[verifier::external_body]
    pub fn string_is_empty(s: &String) -> (b: bool)
        ensures b == (s@.len() == 0)
    { s.is_empty() }

    } // verus!
} // mod
