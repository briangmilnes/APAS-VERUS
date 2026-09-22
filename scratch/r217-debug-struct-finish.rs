// r217: measure what a derived `Debug` prints for a unit struct against the
// `f.debug_struct("X").finish()` body it replaces. The four `*Inv` and `Lnk`
// unit structs in Chap37 and Chap39 use that shape rather than `write!`, so the
// printed text has to be measured separately from
// `scratch/r217-unit-struct-debug.rs`. Plain Rust, no vstd.
// Run: rustc -O scratch/r217-debug-struct-finish.rs -o scratch/r217-debug-struct-finish && scratch/r217-debug-struct-finish

#[derive(Debug)]
pub struct DerivedS;

pub struct BuilderS;

impl std::fmt::Debug for BuilderS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuilderS").finish()
    }
}

fn main() {
    let derived_plain = format!("{:?}", DerivedS);
    let derived_alternate = format!("{:#?}", DerivedS);
    let builder_plain = format!("{:?}", BuilderS);
    let builder_alternate = format!("{:#?}", BuilderS);
    println!("derived {{:?}}   = {:?}", derived_plain);
    println!("derived {{:#?}}  = {:?}", derived_alternate);
    println!("builder {{:?}}   = {:?}", builder_plain);
    println!("builder {{:#?}}  = {:?}", builder_alternate);
    // Both forms print the bare type name under both format specifiers, so
    // replacing an empty `debug_struct(...).finish()` body by the derive leaves
    // the printed text unchanged.
    assert_eq!(derived_plain, "DerivedS");
    assert_eq!(derived_alternate, "DerivedS");
    assert_eq!(builder_plain, "BuilderS");
    assert_eq!(builder_alternate, "BuilderS");
    println!("same shape under both {{:?}} and {{:#?}}: yes");
}
