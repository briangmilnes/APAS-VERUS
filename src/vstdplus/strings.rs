//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! ASCII string and character utilities for verified code.
//! Provides external_body bridges with specs for String/char operations
//! that vstd doesn't cover. All operations assume ASCII input.

pub mod strings {

    use vstd::prelude::*;

    verus! {

    // 6. spec fns

    /// Spec: character is ASCII alphabetic (a-z or A-Z).
    pub open spec fn spec_is_ascii_alpha(c: char) -> bool {
        (c as u32) >= 65 && (c as u32) <= 90     // A-Z
        || (c as u32) >= 97 && (c as u32) <= 122  // a-z
    }

    /// Spec: ASCII lowercase of a character.
    pub open spec fn spec_to_ascii_lowercase_char(c: char) -> char
    {
        if (c as u32) >= 65 && (c as u32) <= 90 {
            // A-Z → a-z: add 32
            (c as u32 + 32) as char
        } else {
            c
        }
    }

    /// Spec: ASCII lowercase of a string (as a sequence of chars).
    pub open spec fn spec_to_ascii_lowercase(s: Seq<char>) -> Seq<char>
    {
        s.map(|_i: int, c: char| spec_to_ascii_lowercase_char(c))
    }

    /// Spec: string push appends one character.
    pub open spec fn spec_string_push(s: Seq<char>, c: char) -> Seq<char>
    {
        s.push(c)
    }

    // 9. impls — exec bridges

    /// Exec: test if character is ASCII alphabetic.
    #[verifier::external_body]
    pub fn is_ascii_alpha(c: char) -> (b: bool)
        ensures b == spec_is_ascii_alpha(c)
    {
        c.is_ascii_alphabetic()
    }

    /// Exec: ASCII lowercase a character.
    #[verifier::external_body]
    pub fn to_ascii_lowercase_char(c: char) -> (lc: char)
        ensures lc == spec_to_ascii_lowercase_char(c)
    {
        c.to_ascii_lowercase()
    }

    /// Exec: push a character onto a String.
    /// View of String is not available in vstd, so we track via ghost Seq<char>.
    #[verifier::external_body]
    pub fn string_push(s: &mut Vec<char>, c: char)
        ensures s@ == old(s)@.push(c)
    {
        s.push(c);
    }

    /// Exec: create empty char buffer.
    pub fn new_char_buf() -> (buf: Vec<char>)
        ensures buf@.len() == 0
    {
        Vec::new()
    }

    /// Exec: check if char buffer is empty.
    pub fn char_buf_is_empty(buf: &Vec<char>) -> (b: bool)
        ensures b == (buf@.len() == 0)
    {
        buf.len() == 0
    }

    /// Exec: clear a char buffer.
    #[verifier::external_body]
    pub fn char_buf_clear(buf: &mut Vec<char>)
        ensures buf@.len() == 0
    {
        buf.clear();
    }

    /// Exec: convert char buffer to String.
    #[verifier::external_body]
    pub fn char_buf_to_string(buf: &Vec<char>) -> (s: String)
    {
        buf.iter().collect()
    }

    /// Exec: convert &str to a Vec<char> for iteration with specs.
    #[verifier::external_body]
    pub fn str_to_chars(s: &str) -> (chars: Vec<char>)
        ensures chars@.len() >= 0
    {
        s.chars().collect()
    }

    /// Exec: ASCII lowercase a string, returned as Vec<char>.
    #[verifier::external_body]
    pub fn to_ascii_lowercase_chars(s: &str) -> (chars: Vec<char>)
        ensures chars@.len() >= 0
    {
        s.to_ascii_lowercase().chars().collect()
    }

    } // verus!
} // mod
