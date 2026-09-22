// r217: measure what a derived `Debug` prints for a unit struct, against the
// constant-string body it replaces. Plain Rust, no vstd.
// Run: rustc -O scratch/r217-unit-struct-debug.rs -o scratch/r217-unit-struct-debug && scratch/r217-unit-struct-debug

#[derive(Debug)]
pub struct DerivedS;

pub struct HandWrittenS;

impl std::fmt::Debug for HandWrittenS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandWrittenS")
    }
}

fn main() {
    let derived_plain = format!("{:?}", DerivedS);
    let derived_alternate = format!("{:#?}", DerivedS);
    let hand_plain = format!("{:?}", HandWrittenS);
    let hand_alternate = format!("{:#?}", HandWrittenS);
    println!("derived {{:?}}   = {:?}", derived_plain);
    println!("derived {{:#?}}  = {:?}", derived_alternate);
    println!("hand    {{:?}}   = {:?}", hand_plain);
    println!("hand    {{:#?}}  = {:?}", hand_alternate);
    // The two differ only in the type name, which is what the replacement changes.
    assert_eq!(derived_plain, "DerivedS");
    assert_eq!(derived_alternate, "DerivedS");
    assert_eq!(hand_plain, "HandWrittenS");
    assert_eq!(hand_alternate, "HandWrittenS");
    println!("same shape under both {{:?}} and {{:#?}}: yes");
}
