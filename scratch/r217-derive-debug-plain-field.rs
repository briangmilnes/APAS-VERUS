// r217: measure what `#[derive(Debug)]` prints for a struct that has fields but
// whose hand-written body printed only the type name. Companion to
// `scratch/r217-derive-debug-non-unit.rs`, which measures the E0277 a field type
// without `Debug` produces. Plain Rust, no vstd.
// Run: rustc -O scratch/r217-derive-debug-plain-field.rs -o scratch/r217-derive-debug-plain-field && scratch/r217-derive-debug-plain-field

#[derive(Debug)]
pub struct DerivedS {
    pub n: u64,
}

pub struct HandWrittenS {
    pub n: u64,
}

impl std::fmt::Debug for HandWrittenS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandWrittenS")
    }
}

fn main() {
    let derived = format!("{:?}", DerivedS { n: 7 });
    let hand = format!("{:?}", HandWrittenS { n: 7 });
    println!("derived = {:?}", derived);
    println!("hand    = {:?}", hand);
    assert_eq!(derived, "DerivedS { n: 7 }");
    assert_eq!(hand, "HandWrittenS");
    println!("the derive prints the fields, the hand body prints the type name only");
}
